use dioxus::prelude::*;
use stayhydated_dioxus::{Project, ProjectSite, StayhydatedSinglePageProjectApp};

const PROJECT: Project = Project::new(
    "frame-capture",
    "Typed routes, scenarios, sizes, and output paths for deterministic captures.",
)
.with_skill_command("npx skills add stayhydated/frame-capture");
const SITE_URL: &str = "https://stayhydated.github.io/frame-capture/";
const RUSTDOC_URL: &str = "https://docs.rs/frame-capture/";
const SOURCE_URL: &str = "https://github.com/stayhydated/frame-capture";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn site() -> ProjectSite {
    ProjectSite::builder()
        .project(PROJECT)
        .site_url(SITE_URL)
        .rustdoc_url(RUSTDOC_URL)
        .source_url(SOURCE_URL)
        .version(VERSION)
        .build()
}

#[component]
pub fn App() -> Element {
    rsx! { StayhydatedSinglePageProjectApp { site: site() } }
}

pub fn route_manifest() -> stayhydated_site::SiteRouteManifest {
    site().single_page_route_manifest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_page_site_has_no_browser_demo() {
        let site = site();

        assert_eq!(site.demo_path(), None);
        assert_eq!(site.rustdoc_url(), RUSTDOC_URL);
        assert_eq!(site.source_url(), SOURCE_URL);
        assert_eq!(
            site.project().skill_command(),
            Some("npx skills add stayhydated/frame-capture")
        );
        assert_eq!(
            route_manifest()
                .application_paths()
                .iter()
                .map(|path| path.as_str())
                .collect::<Vec<_>>(),
            ["/"]
        );
        assert!(
            !route_manifest()
                .static_paths()
                .iter()
                .any(|path| path.as_str() == "/demo/")
        );
    }
}
