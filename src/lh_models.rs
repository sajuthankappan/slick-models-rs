use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct AllAttemptReports {
    best_score: f64,
    best_score_index: usize,

    #[serde(rename = "results")]
    reports: Vec<Report>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Report {
    lighthouse_version: String,
    requested_url: String,
    final_url: String,
    fetch_time: String,
    environment: Environment,
    audits: Audits,
    config_settings: ConfigSettings,
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
#[serde(rename_all = "camelCase")]
pub struct AuditSimple {
    id: String,
    title: String,
    description: String,
    score: Option<f64>,
    warnings: Option<Vec<String>>,
    score_display_mode: Option<String>,
    numeric_value: Option<f64>,
    numeric_unit: Option<String>,
    display_value: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Audit<T> {
    id: String,
    title: String,
    description: String,
    score: Option<f64>,
    warnings: Option<Vec<String>>,
    score_display_mode: Option<String>,
    numeric_value: Option<f64>,
    numeric_unit: Option<String>,
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
#[serde(rename_all = "camelCase")]
pub struct TableHeading {
    key: String,
    item_type: String,
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
#[serde(rename_all = "camelCase")]
pub struct OpportunityColumnHeading {
    key: String,
    value_type: String,
    label: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OpportunityItem {
    url: String,
    total_bytes: Option<i64>,
    wasted_bytes: Option<f64>,
    wasted_ms: Option<i64>,
    wasted_percent: Option<f64>,
    request_start_time: Option<f64>,
    protocol: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct NetworkRequest {
    url: String,
    start_time: Option<f64>,
    end_time: Option<f64>,
    finished: Option<bool>,
    transfer_size: Option<i64>,
    resource_size: i64,
    status_code: i16,
    mime_type: String,
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
#[serde(rename_all = "camelCase")]
pub struct Task {
    duration: f64,
    start_time: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    resource_type: String,
    label: String,
    request_count: i32,
    transfer_size: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Node {
    node: NodeValue,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct NodeValue {
    #[serde(rename = "type")]
    node_type: String,
    path: Option<String>,
    selector: Option<String>,
    snippet: Option<String>,
    node_label: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct ThirdPartyDetail {
    entity: ThirdPartyEntity,
    transfer_size: i64,
    main_thread_time: f64,
    blocking_time: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct ScriptExecutionItem {
    url: String,
    total: f64,
    scripting: f64,
    script_parse_compile: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct WorkBreakdownItem {
    group: String,
    group_label: String,
    duration: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct LatencyItem {
    origin: String,
    server_response_time: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct CachePolicyItem {
    url: String,
    cache_lifetime_ms: i64,
    total_bytes: i64,
    wasted_bytes: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct UserTiming {
    name: String,
    start_time: f64,
    duration: Option<f64>,
    timing_type: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct ConfigSettings {
    throttling_method: String,
    throttling: Throttling,
    audit_mode: bool,
    gather_mode: bool,
    disable_storage_reset: bool,
    emulated_form_factor: String,
    internal_disable_device_screen_emulation: Option<bool>,
    channel: String,
    locale: String,
    only_categories: Vec<String>,
    //budgets: String, //TODO
    //blockedUrlPatterns: String, //TODO
    //additionalTraceCategories: String,
    //extraHeaders
    //precomputedLanternData
    //onlyAudits
    //#[serde(rename = "skipAudits")]
    //skip_audits: Option<bool>,//TODO
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Throttling {
    rtt_ms: i32,
    throughput_kbps: f64,
    request_latency_ms: f64,
    download_throughput_kbps: f64,
    upload_throughput_kbps: i16,
    cpu_slowdown_multiplier: i16,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Categories {
    performance: Performance,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Performance {
    id: String,
    title: String,
    score: f64,
    audit_refs: Vec<AuditRef>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct AuditRef {
    id: String,
    weight: i8,
    group: Option<String>,
}
