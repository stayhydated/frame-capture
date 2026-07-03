use std::collections::BTreeSet;

use thiserror::Error;

use crate::{CaptureItemSpec, CaptureRoute, CaptureRouteIdRef, CaptureScenario, CaptureStateIdRef};

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum CaptureCatalogValidationError {
    #[error("duplicate capture {kind} id `{id}`")]
    DuplicateId { kind: &'static str, id: String },
    #[error("capture route default `{id}` is missing from ROUTES")]
    MissingDefaultRoute { id: String },
    #[error("capture {kind} id `{id}` is not present in the typed {kind} list")]
    UnknownId { kind: &'static str, id: String },
    #[error("capture {kind} value `{value_id}` has metadata id `{spec_id}`")]
    MismatchedId {
        kind: &'static str,
        value_id: String,
        spec_id: String,
    },
    #[error("capture {kind} id `{id}` is missing metadata")]
    MissingMetadata { kind: &'static str, id: String },
}

pub fn validate_capture_routes<R: CaptureRoute>() -> Result<(), CaptureCatalogValidationError> {
    let default_id = R::DEFAULT.spec().id();
    if !R::ROUTES.contains(&R::DEFAULT) {
        return Err(CaptureCatalogValidationError::MissingDefaultRoute {
            id: default_id.to_owned(),
        });
    }

    let mut route_ids = BTreeSet::new();
    for &route in R::ROUTES {
        let id = route.spec().id();
        if !route_ids.insert(id) {
            return Err(duplicate("route", id));
        }

        match R::from_id(id) {
            Ok(parsed) if parsed == route => {},
            Ok(parsed) => {
                return Err(CaptureCatalogValidationError::MismatchedId {
                    kind: "route",
                    value_id: parsed.spec().id().to_owned(),
                    spec_id: id.to_owned(),
                });
            },
            Err(_) => {
                return Err(CaptureCatalogValidationError::UnknownId {
                    kind: "route",
                    id: id.to_owned(),
                });
            },
        }
    }

    let spec_ids = validate_duplicate_specs(
        "route",
        R::ROUTE_SPECS
            .iter()
            .copied()
            .map(|variant| variant.spec.id()),
    )?;
    for variant in R::ROUTE_SPECS {
        let spec = variant.spec;
        if !route_ids.contains(spec.id()) {
            return Err(CaptureCatalogValidationError::UnknownId {
                kind: "route",
                id: spec.id().to_owned(),
            });
        }

        let value_id = variant.route.spec().id();
        let spec_id = variant.spec.id();
        if value_id != spec_id {
            return Err(CaptureCatalogValidationError::MismatchedId {
                kind: "route",
                value_id: value_id.to_owned(),
                spec_id: spec_id.to_owned(),
            });
        }
    }
    ensure_metadata_complete("route", &route_ids, &spec_ids)?;

    Ok(())
}

pub fn validate_capture_scenarios<S: CaptureScenario>() -> Result<(), CaptureCatalogValidationError>
{
    validate_capture_items(
        "scenario",
        S::SCENARIOS.iter().copied().map(|scenario| {
            let id = scenario.id();
            let parsed = S::from_id(id).ok();
            (scenario, id, parsed)
        }),
        S::VARIANTS.iter().copied(),
        S::SPECS,
        S::SCENARIO_SPECS
            .iter()
            .copied()
            .map(|variant| (variant.value.id(), variant.spec)),
        S::spec,
    )
}

fn validate_capture_items<T: Copy + Eq>(
    kind: &'static str,
    typed_items: impl IntoIterator<Item = (T, &'static str, Option<T>)>,
    variant_ids: impl IntoIterator<Item = &'static str>,
    specs: &'static [CaptureItemSpec],
    typed_specs: impl IntoIterator<Item = (&'static str, CaptureItemSpec)>,
    item_spec: fn(T) -> CaptureItemSpec,
) -> Result<(), CaptureCatalogValidationError> {
    let mut item_ids = BTreeSet::new();
    let mut items = Vec::new();
    for (item, id, parsed) in typed_items {
        CaptureStateIdRef::try_new(id).map_err(|_| CaptureCatalogValidationError::UnknownId {
            kind,
            id: id.to_owned(),
        })?;
        if !item_ids.insert(id) {
            return Err(duplicate(kind, id));
        }
        match parsed {
            Some(parsed) if parsed == item => {},
            Some(_) | None => {
                return Err(CaptureCatalogValidationError::UnknownId {
                    kind,
                    id: id.to_owned(),
                });
            },
        }
        items.push((item, id));
    }

    let variant_ids = variant_ids.into_iter().collect::<Vec<_>>();
    validate_duplicate_state_ids(kind, variant_ids.iter().copied())?;
    for id in variant_ids {
        CaptureStateIdRef::try_new(id).map_err(|_| CaptureCatalogValidationError::UnknownId {
            kind,
            id: id.to_owned(),
        })?;
        if !item_ids.contains(id) {
            return Err(CaptureCatalogValidationError::UnknownId {
                kind,
                id: id.to_owned(),
            });
        }
    }

    let spec_ids = validate_item_specs(kind, specs, &item_ids)?;
    ensure_metadata_complete(kind, &item_ids, &spec_ids)?;

    let mut typed_spec_ids = BTreeSet::new();
    for (value_id, spec) in typed_specs {
        if value_id != spec.id() {
            return Err(CaptureCatalogValidationError::MismatchedId {
                kind,
                value_id: value_id.to_owned(),
                spec_id: spec.id().to_owned(),
            });
        }
        if !item_ids.contains(spec.id()) {
            return Err(CaptureCatalogValidationError::UnknownId {
                kind,
                id: spec.id().to_owned(),
            });
        }
        if !typed_spec_ids.insert(spec.id()) {
            return Err(duplicate(kind, spec.id()));
        }
    }
    ensure_metadata_complete(kind, &item_ids, &typed_spec_ids)?;

    for (item, id) in items {
        let spec = item_spec(item);
        if spec.id() != id {
            return Err(CaptureCatalogValidationError::MismatchedId {
                kind,
                value_id: id.to_owned(),
                spec_id: spec.id().to_owned(),
            });
        }
    }

    Ok(())
}

fn validate_item_specs(
    kind: &'static str,
    specs: &'static [CaptureItemSpec],
    item_ids: &BTreeSet<&'static str>,
) -> Result<BTreeSet<&'static str>, CaptureCatalogValidationError> {
    let mut seen = BTreeSet::new();
    for spec in specs {
        let id = spec.id();
        if !seen.insert(id) {
            return Err(duplicate(kind, id));
        }
        if !item_ids.contains(id) {
            return Err(CaptureCatalogValidationError::UnknownId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    Ok(seen)
}

fn validate_duplicate_specs(
    kind: &'static str,
    ids: impl IntoIterator<Item = &'static str>,
) -> Result<BTreeSet<&'static str>, CaptureCatalogValidationError> {
    let mut seen = BTreeSet::new();
    for id in ids {
        CaptureRouteIdRef::try_new(id).map_err(|_| CaptureCatalogValidationError::UnknownId {
            kind,
            id: id.to_owned(),
        })?;
        if !seen.insert(id) {
            return Err(duplicate(kind, id));
        }
    }
    Ok(seen)
}

fn validate_duplicate_state_ids(
    kind: &'static str,
    ids: impl IntoIterator<Item = &'static str>,
) -> Result<(), CaptureCatalogValidationError> {
    let mut seen = BTreeSet::new();
    for id in ids {
        CaptureStateIdRef::try_new(id).map_err(|_| CaptureCatalogValidationError::UnknownId {
            kind,
            id: id.to_owned(),
        })?;
        if !seen.insert(id) {
            return Err(duplicate(kind, id));
        }
    }
    Ok(())
}

fn duplicate(kind: &'static str, id: &str) -> CaptureCatalogValidationError {
    CaptureCatalogValidationError::DuplicateId {
        kind,
        id: id.to_owned(),
    }
}

fn ensure_metadata_complete(
    kind: &'static str,
    item_ids: &BTreeSet<&'static str>,
    metadata_ids: &BTreeSet<&'static str>,
) -> Result<(), CaptureCatalogValidationError> {
    for id in item_ids {
        if !metadata_ids.contains(id) {
            return Err(CaptureCatalogValidationError::MissingMetadata {
                kind,
                id: (*id).to_owned(),
            });
        }
    }

    Ok(())
}
