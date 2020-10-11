use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AllAttemptReports {
    #[serde(rename = "bestScore")]
    best_score: f64,

    #[serde(rename = "bestScoreIndex")]
    best_score_index: usize,

    #[serde(rename = "results")]
    reports: Vec<Report>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Report {
    #[serde(rename = "lighthouseVersion")]
    lighthouse_version: String,

    #[serde(rename = "requestedUrl")]
    requested_url: String,

    #[serde(rename = "finalUrl")]
    final_url: String,

    #[serde(rename = "fetchTime")]
    fetch_time: String,

    #[serde(rename = "environment")]
    environment: Environment,

    audits: Audits,

    #[serde(rename = "configSettings")]
    config_settings: ConfigSettings,

    #[serde(rename = "categories")]
    categories: Categories,
    //TODOs
    //categoryGroups
    //timing
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Environment {}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Audits {
    #[serde(rename = "first-contentful-paint")]
    first_contentful_paint: AuditSimple,

    #[serde(rename = "largest-contentful-paint")]
    largest_contentful_paint: Option<AuditSimple>,

    #[serde(rename = "largest-contentful-paint-element")]
    largest_contentful_paint_element: Option<AuditTable<Node>>,

    #[serde(rename = "first-meaningful-paint")]
    first_meaningful_paint: AuditSimple,

    #[serde(rename = "speed-index")]
    speed_index: AuditSimple,

    #[serde(rename = "total-blocking-time")]
    total_blocking_time: AuditSimple,

    #[serde(rename = "max-potential-fid")]
    max_potential_fid: AuditSimple,

    #[serde(rename = "cumulative-layout-shift")]
    cumulative_layout_shift: Option<AuditSimple>,

    #[serde(rename = "server-response-time")]
    server_response_time: Option<AuditSimple>, // TODO: More fields

    #[serde(rename = "first-cpu-idle")]
    first_cpu_idle: AuditSimple,

    #[serde(rename = "interactive")]
    interactive: AuditSimple,

    #[serde(rename = "network-requests")]
    network_requests: AuditTable<NetworkRequest>,

    #[serde(rename = "network-rtt")]
    network_rtt: AuditTable<NetworkRttItem>,

    #[serde(rename = "main-thread-tasks")]
    main_thread_tasks: AuditTable<Task>,

    #[serde(rename = "metrics")]
    metrics: AuditSimple,

    #[serde(rename = "resource-summary")]
    resource_summary: AuditTable<Resource>,

    #[serde(rename = "third-party-summary")]
    third_party_summary: AuditTable<ThirdPartyDetail>,

    #[serde(rename = "screenshot-thumbnails")]
    screenshot_thumbnails: Audit<Filmstrip>,

    #[serde(rename = "uses-responsive-images")]
    uses_responsive_images: Audit<Opportunity>,

    #[serde(rename = "uses-optimized-images")]
    uses_optimized_images: Audit<Opportunity>,

    #[serde(rename = "uses-webp-images")]
    uses_webp_images: Audit<Opportunity>,

    #[serde(rename = "offscreen-images")]
    offscreen_images: Audit<Opportunity>,

    #[serde(rename = "uses-http2")]
    uses_http2: Option<Audit<Opportunity>>,

    #[serde(rename = "bootup-time")]
    bootup_time: Option<AuditTable<ScriptExecutionItem>>,

    #[serde(rename = "mainthread-work-breakdown")]
    main_thread_work_breakdown: Option<AuditTable<WorkBreakdownItem>>,

    #[serde(rename = "uses-rel-preconnect")]
    uses_rel_preconnect: Option<AuditSimple>,

    #[serde(rename = "network-server-latency")]
    network_server_latency: Option<AuditTable<LatencyItem>>,

    #[serde(rename = "unminified-css")]
    unminified_css: Option<Audit<Opportunity>>,

    #[serde(rename = "unminified-javascript")]
    unminified_javascript: Option<Audit<Opportunity>>,

    #[serde(rename = "unused-css-rules")]
    unused_css_rules: Option<Audit<Opportunity>>,

    #[serde(rename = "unused-javascript")]
    unused_javascript: Option<Audit<Opportunity>>,

    #[serde(rename = "render-blocking-resources")]
    render_blocking_resources: Option<Audit<Opportunity>>,

    #[serde(rename = "uses-long-cache-ttl")]
    uses_long_cache_ttl: Option<AuditTable<CachePolicyItem>>,

    #[serde(rename = "user-timings")]
    user_timings: Option<AuditTable<UserTiming>>,

    //TODOs
    //layout-shift-elements
    //
    //total-byte-weight
    //uses-text-compression
    //efficient-animated-content
    //dom-size
    //no-document-write
    //
    //uses-passive-event-listeners
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct AuditSimple {
    id: String,
    title: String,
    description: String,
    score: Option<f64>,
    warnings: Option<Vec<String>>,

    #[serde(rename = "scoreDisplayMode")]
    score_display_mode: Option<String>,

    #[serde(rename = "numericValue")]
    numeric_value: Option<f64>,

    #[serde(rename = "numericUnit")]
    numeric_unit: Option<String>,

    #[serde(rename = "displayValue")]
    display_value: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Audit<T> {
    id: String,
    title: String,
    description: String,
    score: Option<f64>,
    warnings: Option<Vec<String>>,

    #[serde(rename = "scoreDisplayMode")]
    score_display_mode: Option<String>,

    #[serde(rename = "numericValue")]
    numeric_value: Option<f64>,

    #[serde(rename = "numericUnit")]
    numeric_unit: Option<String>,

    #[serde(rename = "displayValue")]
    display_value: Option<String>,

    details: T,
}

pub type AuditTable<T> = Audit<Table<T>>;

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Table<T> {
    headings: Option<Vec<TableHeading>>,
    items: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct TableHeading {
    key: String,

    #[serde(rename = "itemType")]
    item_type: String,

    #[serde(rename = "displayUnit")]
    display_unit: Option<String>,

    text: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Opportunity {
    headings: Option<Vec<OpportunityColumnHeading>>,
    items: Vec<OpportunityItem>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct OpportunityColumnHeading {
    key: String,

    #[serde(rename = "valueType")]
    value_type: String,

    label: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct OpportunityItem {
    url: String,

    #[serde(rename = "totalBytes")]
    total_bytes: Option<i64>,

    #[serde(rename = "wastedBytes")]
    wasted_bytes: Option<f64>,

    #[serde(rename = "wastedMs")]
    wasted_ms: Option<i64>,

    #[serde(rename = "wastedPercent")]
    wasted_percent: Option<f64>,

    #[serde(rename = "requestStartTime")]
    request_start_time: Option<f64>,

    #[serde(rename = "protocol")]
    protocol: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NetworkRequest {
    url: String,

    #[serde(rename = "startTime")]
    start_time: Option<f64>,

    #[serde(rename = "endTime")]
    end_time: Option<f64>,

    #[serde(rename = "finished")]
    finished: Option<bool>,

    #[serde(rename = "transferSize")]
    transfer_size: Option<i64>,

    #[serde(rename = "resourceSize")]
    resource_size: i64,

    #[serde(rename = "statusCode")]
    status_code: i16,

    #[serde(rename = "mimeType")]
    mime_type: String,

    #[serde(rename = "resourceType")]
    resource_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NetworkRttItem {
    origin: String,
    rtt: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Task {
    duration: f64,
    #[serde(rename = "startTime")]
    start_time: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Resource {
    #[serde(rename = "resourceType")]
    resource_type: String,
    #[serde(rename = "label")]
    label: String,
    #[serde(rename = "requestCount")]
    request_count: i32,
    #[serde(rename = "transferSize")]
    transfer_size: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Node {
    node: NodeValue,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NodeValue {
    #[serde(rename = "type")]
    node_type: String,
    path: Option<String>,
    selector: Option<String>,
    snippet: Option<String>,
    #[serde(rename = "nodeLabel")]
    node_label: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ThirdPartyDetail {
    entity: ThirdPartyEntity,
    #[serde(rename = "transferSize")]
    transfer_size: i64,
    #[serde(rename = "mainThreadTime")]
    main_thread_time: f64,
    #[serde(rename = "blockingTime")]
    blocking_time: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ThirdPartyEntity {
    #[serde(rename = "type")]
    entity_type: String,
    text: String,
    url: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Filmstrip {
    scale: i64,
    items: Vec<FilmstripItem>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct FilmstripItem {
    timing: i64,
    timestamp: f64,
    data: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ScriptExecutionItem {
    url: String,
    total: f64,
    scripting: f64,
    #[serde(rename = "scriptParseCompile")]
    script_parse_compile: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct WorkBreakdownItem {
    group: String,
    #[serde(rename = "groupLabel")]
    group_label: String,
    duration: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct LatencyItem {
    origin: String,
    #[serde(rename = "serverResponseTime")]
    server_response_time: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct CachePolicyItem {
    url: String,

    #[serde(rename = "cacheLifetimeMs")]
    cache_lifetime_ms: i64,

    #[serde(rename = "totalBytes")]
    total_bytes: i64,

    #[serde(rename = "wastedBytes")]
    wasted_bytes: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct UserTiming {
    name: String,

    #[serde(rename = "startTime")]
    start_time: f64,

    duration: Option<f64>,

    #[serde(rename = "timingType")]
    timing_type: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ConfigSettings {
    #[serde(rename = "throttlingMethod")]
    throttling_method: String,

    #[serde(rename = "throttling")]
    throttling: Throttling,

    #[serde(rename = "auditMode")]
    audit_mode: bool,

    #[serde(rename = "gatherMode")]
    gather_mode: bool,

    #[serde(rename = "disableStorageReset")]
    disable_storage_reset: bool,

    #[serde(rename = "emulatedFormFactor")]
    emulated_form_factor: String,

    #[serde(rename = "internalDisableDeviceScreenEmulation")]
    internal_disable_device_screen_emulation: Option<bool>,

    channel: String,
    //budgets: String, //TODO
    locale: String,
    //blockedUrlPatterns: String, //TODO
    //additionalTraceCategories: String,
    //extraHeaders
    //precomputedLanternData
    //onlyAudits
    #[serde(rename = "onlyCategories")]
    only_categories: Vec<String>,
    //#[serde(rename = "skipAudits")]
    //skip_audits: Option<bool>,//TODO
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Throttling {
    #[serde(rename = "rttMs")]
    rtt_ms: i32,

    #[serde(rename = "throughputKbps")]
    throughput_kbps: f64,

    #[serde(rename = "requestLatencyMs")]
    request_latency_ms: f64,

    #[serde(rename = "downloadThroughputKbps")]
    download_throughput_kbps: f64,

    #[serde(rename = "uploadThroughputKbps")]
    upload_throughput_kbps: i16,

    #[serde(rename = "cpuSlowdownMultiplier")]
    cpu_slowdown_multiplier: i16,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Categories {
    #[serde(rename = "performance")]
    performance: Performance,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Performance {
    id: String,
    title: String,
    score: f64,

    #[serde(rename = "auditRefs")]
    audit_refs: Vec<AuditRef>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct AuditRef {
    id: String,
    weight: i8,
    group: Option<String>,
}
