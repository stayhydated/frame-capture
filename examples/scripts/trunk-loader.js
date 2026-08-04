const LOADER_STYLE_ID = "__stayhydated_wasm_loader_styles";
const DEFAULT_LOADER_ID = "wasm-demo-loader";
const DEFAULT_PROGRESS_ID = "wasm-demo-progress";
const DEFAULT_DEMO_NAME = "Web Demo";
const RUNTIME_CONFIG_SELECTOR = "[data-wasm-demo-config]";

const LOADER_STYLES = `
.wasm-demo-loader,
.wasm-demo-loader[data-state="ready"] {
  --wasm-loader-accent: rgb(255 255 255);
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: grid;
  place-items: center;
  padding: clamp(1.25rem, 4vw, 3rem);
  overflow: hidden;
  isolation: isolate;
  background:
    radial-gradient(circle at 50% 5%, rgba(255, 255, 255, 0.055), transparent 34%),
    #000;
  transition: opacity 180ms ease;
  pointer-events: none;
}

.wasm-demo-loader {
  pointer-events: auto;
}

.wasm-demo-loader::before,
.wasm-demo-loader::after {
  position: absolute;
  z-index: 0;
  pointer-events: none;
  content: "";
}

.wasm-demo-loader::before {
  inset: 0;
  opacity: 0.56;
  background:
    linear-gradient(
      90deg,
      transparent 49.8%,
      color-mix(in srgb, var(--wasm-loader-accent) 13%, transparent) 50%,
      transparent 50.2%
    ),
    repeating-linear-gradient(
      0deg,
      transparent 0 23px,
      color-mix(in srgb, var(--wasm-loader-accent) 4.5%, transparent) 24px
    ),
    repeating-linear-gradient(
      90deg,
      transparent 0 47px,
      color-mix(in srgb, var(--wasm-loader-accent) 4.5%, transparent) 48px
    );
  mask-image: linear-gradient(to bottom, #000, transparent 20%, transparent 80%, #000);
}

.wasm-demo-loader::after {
  inset: 0.8rem;
  border: 1px solid rgba(255, 255, 255, 0.16);
  clip-path: polygon(
    0 2.5rem,
    2.5rem 0,
    calc(100% - 2.5rem) 0,
    100% 2.5rem,
    100% calc(100% - 2.5rem),
    calc(100% - 2.5rem) 100%,
    2.5rem 100%,
    0 calc(100% - 2.5rem)
  );
  box-shadow: inset 0 0 2rem rgba(255, 255, 255, 0.04);
}

.wasm-demo-loader[data-state="ready"] {
  opacity: 0;
}

.wasm-demo-loader[data-state="error"] {
  pointer-events: auto;
}

.wasm-loader-card {
  position: relative;
  z-index: 1;
  width: min(31rem, 100%);
  padding: clamp(2rem, 7vw, 3.25rem);
  overflow: hidden;
  isolation: isolate;
  background: var(--wasm-loader-accent);
  clip-path: polygon(
    0 0,
    calc(100% - 2.5rem) 0,
    100% 2.5rem,
    100% 100%,
    2.5rem 100%,
    0 calc(100% - 2.5rem)
  );
  color: #fff;
  filter: drop-shadow(
    0 0 1.1rem color-mix(in srgb, var(--wasm-loader-accent) 42%, transparent)
  );
  font-family: Inter, ui-sans-serif, system-ui, sans-serif;
  text-align: center;
}

.wasm-loader-card::before,
.wasm-loader-card::after {
  position: absolute;
  pointer-events: none;
  content: "";
}

.wasm-loader-card::before {
  z-index: -2;
  inset: 1px;
  background:
    linear-gradient(
      122deg,
      transparent 0 57%,
      color-mix(in srgb, var(--wasm-loader-accent) 13%, transparent) 57.2%,
      transparent 66%
    ),
    repeating-linear-gradient(
      0deg,
      transparent 0 31px,
      color-mix(in srgb, var(--wasm-loader-accent) 7%, transparent) 32px
    ),
    repeating-linear-gradient(
      90deg,
      transparent 0 31px,
      color-mix(in srgb, var(--wasm-loader-accent) 6%, transparent) 32px
    ),
    radial-gradient(
      circle at 82% 12%,
      color-mix(in srgb, var(--wasm-loader-accent) 12%, transparent),
      transparent 42%
    ),
    #000;
  clip-path: inherit;
}

.wasm-loader-card::after {
  z-index: -1;
  inset: 0.85rem;
  border-top: 1px solid color-mix(in srgb, var(--wasm-loader-accent) 70%, transparent);
  border-bottom: 1px solid color-mix(in srgb, var(--wasm-loader-accent) 50%, transparent);
  opacity: 0.86;
  clip-path: polygon(0 0, 34% 0, 39% 0.35rem, 61% 0.35rem, 66% 0, 100% 0, 100% 100%, 0 100%);
}

.wasm-loader-kicker {
  color: var(--wasm-loader-accent);
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
  font-size: 0.72rem;
  font-weight: 800;
  letter-spacing: 0.2em;
  line-height: 1;
  text-shadow:
    0 0 0.8rem color-mix(in srgb, var(--wasm-loader-accent) 58%, transparent);
  text-transform: uppercase;
}

.wasm-loader-title {
  margin: 0.75rem 0 0;
  color: var(--wasm-loader-accent);
  font-size: clamp(2rem, 7vw, 3rem);
  font-weight: 900;
  letter-spacing: -0.055em;
  line-height: 0.92;
  text-shadow:
    0 0 0.08rem #fff,
    0 0 0.8rem color-mix(in srgb, var(--wasm-loader-accent) 68%, transparent),
    0.12rem 0.14rem 0 rgba(0, 0, 0, 0.86);
  text-transform: uppercase;
}

.wasm-status-line {
  display: none;
  margin: 1.2rem 0 0;
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
  font-size: 0.78rem;
  font-weight: 800;
  letter-spacing: 0.08em;
  line-height: 1.2;
  text-transform: uppercase;
}

.wasm-status-line::before {
  margin-right: 0.45rem;
  content: "//";
}

.wasm-demo-loader[data-state="loading"] .wasm-status-line[data-state="loading"],
.wasm-demo-loader[data-state="error"] .wasm-status-line[data-state="error"] {
  display: block;
}

.wasm-demo-loader[data-state="loading"] .wasm-status-line[data-state="loading"] {
  color: var(--wasm-loader-accent);
  text-shadow:
    0 0 0.8rem color-mix(in srgb, var(--wasm-loader-accent) 46%, transparent);
}

.wasm-demo-loader[data-state="error"] .wasm-status-line[data-state="error"] {
  color: var(--wasm-loader-accent);
  text-shadow:
    0 0 0.8rem color-mix(in srgb, var(--wasm-loader-accent) 50%, transparent);
}

@media (prefers-reduced-motion: reduce) {
  .wasm-demo-loader,
  .wasm-demo-loader[data-state="ready"] {
    transition: none;
  }
}
`;

function ensureLoaderStyle() {
  if (document.getElementById(LOADER_STYLE_ID)) {
    return;
  }

  const style = document.createElement("style");
  style.id = LOADER_STYLE_ID;
  style.textContent = LOADER_STYLES;
  document.head.append(style);
}

function buildLoaderMarkup(loaderId, progressId, demoName) {
  const loader = document.createElement("div");
  loader.id = loaderId;
  loader.className = "wasm-demo-loader";
  loader.dataset.state = "loading";

  const card = document.createElement("div");
  card.className = "wasm-loader-card";
  const kicker = document.createElement("div");
  kicker.className = "wasm-loader-kicker";
  kicker.textContent = "Browser demo";
  const title = document.createElement("h1");
  title.className = "wasm-loader-title";
  title.textContent = demoName;
  const loading = document.createElement("p");
  loading.className = "wasm-status-line";
  loading.dataset.state = "loading";
  loading.id = progressId;
  loading.textContent = "Loading demo...";
  const error = document.createElement("p");
  error.className = "wasm-status-line";
  error.dataset.state = "error";
  error.textContent = "The demo failed to start.";

  card.append(kicker, title, loading, error);
  loader.append(card);
  return loader;
}

function ensureLoader(loaderId, progressId, demoName) {
  ensureLoaderStyle();
  let loader = document.getElementById(loaderId);
  if (!loader) {
    loader = buildLoaderMarkup(loaderId, progressId, demoName);
    document.body.append(loader);
  }
  return { loader, progress: document.getElementById(progressId) };
}

function setProgress(progress, current, total) {
  if (!progress) {
    return;
  }
  const percent = total
    ? ` ${Math.max(0, Math.min(100, Math.round((current / total) * 100)))}%`
    : "";
  progress.textContent = `Loading demo...${percent}`;
}

function readRuntimeConfig() {
  const link = document.querySelector(RUNTIME_CONFIG_SELECTOR);
  return {
    demoName: link?.dataset.wasmDemoName ?? DEFAULT_DEMO_NAME,
    loaderId: link?.dataset.wasmLoaderId ?? DEFAULT_LOADER_ID,
    progressId: link?.dataset.wasmProgressId ?? DEFAULT_PROGRESS_ID,
    bootstrapModule: link?.dataset.wasmBootstrapModule ?? null,
    bootstrapExport: link?.dataset.wasmBootstrapExport ?? null,
  };
}

async function runBootstrapModule(modulePath, exportName) {
  const module = await import(modulePath);
  if (typeof module.default === "function") {
    await module.default();
  }
  if (exportName) {
    const bootstrap = module[exportName];
    if (typeof bootstrap !== "function") {
      throw new Error(`wasm demo bootstrap export "${exportName}" is not a function`);
    }
    await bootstrap();
  }
}

export default function wasmDemoInitializer() {
  const config = readRuntimeConfig();
  const { loader, progress } = ensureLoader(config.loaderId, config.progressId, config.demoName);

  return {
    onStart() {
      loader.dataset.state = "loading";
      setProgress(progress, 0, 0);
    },
    onProgress({ current, total }) {
      setProgress(progress, current, total);
    },
    async onSuccess() {
      try {
        if (config.bootstrapModule) {
          await runBootstrapModule(config.bootstrapModule, config.bootstrapExport);
        }
        loader.dataset.state = "ready";
      } catch (error) {
        loader.dataset.state = "error";
        console.error(`${config.demoName} demo failed to initialize`, error);
      }
    },
    onFailure(error) {
      loader.dataset.state = "error";
      console.error(`${config.demoName} demo failed to load`, error);
    },
  };
}
