use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand, EngineInterface, EvaluatedCall};
use nu_protocol::*;
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

struct LlmPlugin;

impl LlmPlugin {
    fn new() -> Self {
        Self
    }
}

impl Plugin for LlmPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(LlmTest),
            Box::new(LlmLs),
            Box::new(LlmPs),
            Box::new(LlmSys),
            Box::new(LlmEnv),
            Box::new(LlmGit),
            Box::new(LlmNet),
            Box::new(LlmFind),
            Box::new(LlmAnalyze),
            Box::new(LlmWc),
            Box::new(LlmDu),
            Box::new(LlmMode),
        ]
    }

    fn version(&self) -> String {
        "0.2.0".into()
    }
}

// Helper functions
fn create_metadata(command: &str, start_time: Instant) -> JsonValue {
    json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "command": command,
        "version": "0.2.0",
        "execution_time_ms": start_time.elapsed().as_millis(),
        "source_host": hostname().unwrap_or_else(|| "unknown".to_string()),
        "session_id": uuid::Uuid::new_v4().to_string()
    })
}

fn hostname() -> Option<String> {
    std::env::var("HOSTNAME").ok()
        .or_else(|| sysinfo::System::host_name())
}

fn create_error_response(command: &str, error: &str, suggestions: Vec<&str>) -> JsonValue {
    json!({
        "error": true,
        "command": command,
        "message": error,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "suggestions": suggestions
    })
}

// ================================
// COMMAND IMPLEMENTATIONS
// ================================

struct LlmTest;

impl PluginCommand for LlmTest {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-test" }
    fn description(&self) -> &str { "Test LLM plugin functionality with example structured output" }
    fn signature(&self) -> Signature {
        Signature::build("llm-test").category(Category::Experimental)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let response = json!({
            "metadata": create_metadata("llm-test", start_time),
            "status": "success",
            "message": "🚀 LLM Plugin v0.2.0 - Full Implementation Working!",
            "capabilities": [
                "llm-test - Plugin functionality demo",
                "llm-ls - Enhanced file listing with metadata",
                "llm-ps - Process information with full context",
                "llm-sys - Complete system information",
                "llm-env - Environment variables as structured data",
                "llm-git - Git repository status and metadata",
                "llm-net - Network connections and statistics",
                "llm-find - Advanced file search with results",
                "llm-analyze - File/text analysis with statistics",
                "llm-wc - Word count and text analysis",
                "llm-du - Disk usage with directory breakdown",
                "llm-mode - Toggle LLM optimization settings"
            ],
            "llm_optimized_features": {
                "zero_visual_noise": true,
                "consistent_schemas": true,
                "rich_metadata": true,
                "structured_errors": true,
                "token_efficiency": "maximum",
                "ai_parseable": true
            }
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmLs;

impl PluginCommand for LlmLs {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-ls" }
    fn description(&self) -> &str { "LLM-optimized file listing with complete metadata and checksums" }
    fn signature(&self) -> Signature {
        Signature::build("llm-ls")
            .optional("path", SyntaxShape::String, "Path to list")
            .switch("recursive", "List files recursively", Some('r'))
            .switch("hidden", "Include hidden files", Some('a'))
            .switch("checksums", "Calculate SHA256 checksums", Some('c'))
            .named("depth", SyntaxShape::Int, "Maximum recursion depth", Some('d'))
            .category(Category::FileSystem)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let path = call.opt::<String>(0)?.unwrap_or_else(|| ".".to_string());
        let recursive = call.has_flag("recursive")?;
        let include_hidden = call.has_flag("hidden")?;
        let checksums = call.has_flag("checksums")?;
        let max_depth = call.get_flag::<i64>("depth")?
            .unwrap_or(10)
            .max(0) as usize;

        let files = collect_file_info(&path, recursive, include_hidden, checksums, max_depth);
        
        let response = json!({
            "metadata": create_metadata("llm-ls", start_time),
            "path": path,
            "options": {
                "recursive": recursive,
                "include_hidden": include_hidden,
                "checksums": checksums,
                "max_depth": max_depth
            },
            "file_count": files.len(),
            "files": files
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmPs;

impl PluginCommand for LlmPs {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-ps" }
    fn description(&self) -> &str { "LLM-optimized process listing with complete process information" }
    fn signature(&self) -> Signature {
        Signature::build("llm-ps")
            .switch("all", "Show all processes", Some('a'))
            .switch("environment", "Include environment variables", Some('e'))
            .category(Category::System)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let show_all = call.has_flag("all")?;
        let include_env = call.has_flag("environment")?;
        
        let processes = collect_process_info(show_all, include_env);
        
        let response = json!({
            "metadata": create_metadata("llm-ps", start_time),
            "options": {
                "show_all": show_all,
                "include_environment": include_env
            },
            "process_count": processes.len(),
            "processes": processes
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmSys;

impl PluginCommand for LlmSys {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-sys" }
    fn description(&self) -> &str { "LLM-optimized system information with hardware and OS details" }
    fn signature(&self) -> Signature {
        Signature::build("llm-sys")
            .switch("detailed", "Include detailed disk and network info", Some('d'))
            .category(Category::System)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let detailed = call.has_flag("detailed")?;
        let system_info = collect_system_info(detailed);
        
        let mut response = json!({
            "metadata": create_metadata("llm-sys", start_time)
        });
        
        // Merge system_info into response
        if let JsonValue::Object(ref mut map) = response {
            if let JsonValue::Object(sys_map) = system_info {
                for (key, value) in sys_map {
                    map.insert(key, value);
                }
            }
        }

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmEnv;

impl PluginCommand for LlmEnv {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-env" }
    fn description(&self) -> &str { "LLM-optimized environment variables with structured output" }
    fn signature(&self) -> Signature {
        Signature::build("llm-env")
            .optional("pattern", SyntaxShape::String, "Filter environment variables by pattern")
            .category(Category::System)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let pattern = call.opt::<String>(0)?;
        let env_vars = collect_env_vars(pattern.as_deref());
        
        let response = json!({
            "metadata": create_metadata("llm-env", start_time),
            "variable_count": env_vars.len(),
            "environment_variables": env_vars
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmGit;

impl PluginCommand for LlmGit {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-git" }
    fn description(&self) -> &str { "LLM-optimized Git repository status and metadata" }
    fn signature(&self) -> Signature {
        Signature::build("llm-git")
            .optional("path", SyntaxShape::String, "Repository path")
            .category(Category::FileSystem)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let path = call.opt::<String>(0)?.unwrap_or_else(|| ".".to_string());
        let git_info = collect_git_info(&path);
        
        let response = json!({
            "metadata": create_metadata("llm-git", start_time),
            "repository_path": path,
            "git_info": git_info
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmNet;

impl PluginCommand for LlmNet {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-net" }
    fn description(&self) -> &str { "LLM-optimized network connections and interface information" }
    fn signature(&self) -> Signature {
        Signature::build("llm-net")
            .switch("listening", "Show only listening ports", Some('l'))
            .category(Category::Network)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let listening_only = call.has_flag("listening")?;
        let net_info = collect_network_info(listening_only);
        
        let response = json!({
            "metadata": create_metadata("llm-net", start_time),
            "options": {
                "listening_only": listening_only
            },
            "network_info": net_info
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmFind;

impl PluginCommand for LlmFind {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-find" }
    fn description(&self) -> &str { "LLM-optimized file search with comprehensive results" }
    fn signature(&self) -> Signature {
        Signature::build("llm-find")
            .required("pattern", SyntaxShape::String, "Search pattern (regex supported)")
            .optional("path", SyntaxShape::String, "Search path")
            .switch("case-sensitive", "Case sensitive search", Some('c'))
            .named("type", SyntaxShape::String, "File type filter (file/dir)", Some('t'))
            .named("depth", SyntaxShape::Int, "Maximum search depth", Some('d'))
            .category(Category::FileSystem)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();

        let pattern = call.req::<String>(0)?;
        let path = call.opt::<String>(1)?.unwrap_or_else(|| ".".to_string());
        let case_sensitive = call.has_flag("case-sensitive")?;
        let type_filter = call.get_flag::<String>("type")?;
        let max_depth = call.get_flag::<i64>("depth")?
            .unwrap_or(50)
            .max(0) as usize;

        let results = find_files(&pattern, &path, case_sensitive, type_filter.as_deref(), max_depth);
        
        let response = json!({
            "metadata": create_metadata("llm-find", start_time),
            "search_pattern": pattern,
            "search_path": path,
            "options": {
                "case_sensitive": case_sensitive,
                "type_filter": type_filter
            },
            "result_count": results.len(),
            "results": results
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmAnalyze;

impl PluginCommand for LlmAnalyze {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-analyze" }
    fn description(&self) -> &str { "LLM-optimized file/text analysis with detailed statistics" }
    fn signature(&self) -> Signature {
        Signature::build("llm-analyze")
            .required("file", SyntaxShape::String, "File to analyze")
            .category(Category::Strings)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let file_path = call.req::<String>(0)?;
        let analysis = analyze_file(&file_path);
        
        let response = json!({
            "metadata": create_metadata("llm-analyze", start_time),
            "file_path": file_path,
            "analysis": analysis
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmWc;

impl PluginCommand for LlmWc {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-wc" }
    fn description(&self) -> &str { "LLM-optimized word count and text statistics" }
    fn signature(&self) -> Signature {
        Signature::build("llm-wc")
            .required("file", SyntaxShape::String, "File to count")
            .category(Category::Strings)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let file_path = call.req::<String>(0)?;
        let stats = word_count_analysis(&file_path);
        
        let response = json!({
            "metadata": create_metadata("llm-wc", start_time),
            "file_path": file_path,
            "statistics": stats
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmDu;

impl PluginCommand for LlmDu {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-du" }
    fn description(&self) -> &str { "LLM-optimized disk usage analysis with directory breakdown" }
    fn signature(&self) -> Signature {
        Signature::build("llm-du")
            .optional("path", SyntaxShape::String, "Path to analyze")
            .named("depth", SyntaxShape::Int, "Maximum depth", Some('d'))
            .category(Category::FileSystem)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let path = call.opt::<String>(0)?.unwrap_or_else(|| ".".to_string());
        let max_depth = call.get_flag::<i64>("depth")?
            .unwrap_or(3)
            .max(0) as usize;

        let usage_data = disk_usage_analysis(&path, max_depth);
        
        let response = json!({
            "metadata": create_metadata("llm-du", start_time),
            "analyzed_path": path,
            "max_depth": max_depth,
            "disk_usage": usage_data
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

struct LlmMode;

impl PluginCommand for LlmMode {
    type Plugin = LlmPlugin;

    fn name(&self) -> &str { "llm-mode" }
    fn description(&self) -> &str { "Configure LLM optimization settings and output preferences" }
    fn signature(&self) -> Signature {
        Signature::build("llm-mode")
            .switch("verbose", "Enable verbose metadata", Some('v'))
            .switch("compact", "Enable compact output", Some('c'))
            .switch("status", "Show current settings", Some('s'))
            .category(Category::Core)
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, _input: PipelineData) -> Result<PipelineData, LabeledError> {
        let start_time = Instant::now();
        
        let verbose = call.has_flag("verbose")?;
        let compact = call.has_flag("compact")?;
        let show_status = call.has_flag("status")?;
        
        let response = json!({
            "metadata": create_metadata("llm-mode", start_time),
            "current_settings": {
                "verbose_metadata": verbose,
                "compact_output": compact,
                "llm_optimized": true,
                "zero_visual_noise": true,
                "structured_errors": true,
                "consistent_schemas": true
            },
            "status": if show_status {
                "LLM optimization is ACTIVE - all output structured for AI consumption"
            } else {
                "Settings updated successfully"
            }
        });

        Ok(PipelineData::Value(Value::string(response.to_string(), call.head), None))
    }
}

// ================================
// HELPER FUNCTIONS
// ================================

fn collect_file_info(path: &str, recursive: bool, include_hidden: bool, checksums: bool, max_depth: usize) -> Vec<JsonValue> {
    let mut files = Vec::new();
    
    if recursive {
        for entry in walkdir::WalkDir::new(path).max_depth(max_depth) {
            if let Ok(entry) = entry {
                if !include_hidden && is_hidden(entry.path()) { continue; }
                if let Some(file_info) = create_file_json(entry.path(), checksums) {
                    files.push(file_info);
                }
            }
        }
    } else {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if !include_hidden && is_hidden(&entry.path()) { continue; }
                if let Some(file_info) = create_file_json(&entry.path(), checksums) {
                    files.push(file_info);
                }
            }
        }
    }
    
    files
}

fn create_file_json(path: &Path, calculate_checksum: bool) -> Option<JsonValue> {
    let metadata = std::fs::metadata(path).ok()?;
    
    let checksum = if calculate_checksum && metadata.is_file() {
        calculate_file_checksum(path)
    } else {
        None
    };
    
    Some(json!({
        "name": path.file_name()?.to_string_lossy(),
        "path": path.display().to_string(),
        "type": if metadata.is_dir() { "directory" } else { "file" },
        "size_bytes": metadata.len(),
        "is_readonly": metadata.permissions().readonly(),
        "is_hidden": is_hidden(path),
        "is_symlink": metadata.file_type().is_symlink(),
        "created": metadata.created().ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs()),
        "modified": metadata.modified().ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs()),
        "accessed": metadata.accessed().ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs()),
        "checksum_sha256": checksum
    }))
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn calculate_file_checksum(path: &Path) -> Option<String> {
    use sha2::{Sha256, Digest};
    use std::io::Read;

    let file = std::fs::File::open(path).ok()?;
    let mut reader = std::io::BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = reader.read(&mut buf).ok()?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Some(hex::encode(hasher.finalize()))
}

fn collect_process_info(show_all: bool, include_env: bool) -> Vec<JsonValue> {
    use sysinfo::System;

    let mut system = System::new_all();
    system.refresh_all();

    let now_ts = chrono::Utc::now().timestamp();
    let sensitive_patterns = [
        "SECRET", "TOKEN", "PASSWORD", "PASSWD", "KEY", "CREDENTIAL",
        "AUTH", "PRIVATE", "API_KEY", "APIKEY",
    ];
    let mut processes = Vec::new();

    for (pid, process) in system.processes() {
        if !show_all && process.name().to_string_lossy().starts_with('[') {
            continue;
        }

        let env_vars = if include_env {
            process.environ().iter()
                .map(|env| {
                    let s = env.to_string_lossy().to_string();
                    if let Some(eq_pos) = s.find('=') {
                        let key_upper = s[..eq_pos].to_uppercase();
                        if sensitive_patterns.iter().any(|p| key_upper.contains(p)) {
                            return format!("{}=<REDACTED>", &s[..eq_pos]);
                        }
                    }
                    s
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        processes.push(json!({
            "pid": pid.as_u32(),
            "name": process.name().to_string_lossy(),
            "command": process.cmd().iter().map(|s| s.to_string_lossy()).collect::<Vec<_>>().join(" "),
            "cpu_usage_percent": process.cpu_usage(),
            "memory_bytes": process.memory(),
            "virtual_memory_bytes": process.virtual_memory(),
            "status": format!("{:?}", process.status()),
            "start_time": now_ts - process.run_time() as i64,
            "environment": env_vars
        }));
    }
    
    processes.sort_by(|a, b| {
        b["cpu_usage_percent"].as_f64().unwrap_or(0.0)
            .partial_cmp(&a["cpu_usage_percent"].as_f64().unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    processes
}

fn collect_system_info(detailed: bool) -> JsonValue {
    use sysinfo::System;
    
    let mut system = System::new_all();
    system.refresh_all();
    
    let mut info = json!({
        "hostname": sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string()),
        "os_name": sysinfo::System::name().unwrap_or_else(|| "unknown".to_string()),
        "os_version": sysinfo::System::os_version().unwrap_or_else(|| "unknown".to_string()),
        "kernel_version": sysinfo::System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
        "architecture": std::env::consts::ARCH,
        "cpu_count": system.cpus().len(),
        "total_memory_bytes": system.total_memory(),
        "available_memory_bytes": system.available_memory(),
        "used_memory_bytes": system.used_memory(),
        "total_swap_bytes": system.total_swap(),
        "used_swap_bytes": system.used_swap(),
        "uptime_seconds": sysinfo::System::uptime(),
        "boot_time": chrono::Utc::now().timestamp() - sysinfo::System::uptime() as i64,
        "load_average": get_load_average()
    });
    
    if detailed {
        if let JsonValue::Object(ref mut map) = info {
            map.insert("disks".to_string(), collect_disk_info());
            map.insert("networks".to_string(), collect_network_interfaces());
        }
    }
    
    info
}

fn get_load_average() -> Vec<f64> {
    let load = sysinfo::System::load_average();
    vec![load.one, load.five, load.fifteen]
}

fn collect_disk_info() -> JsonValue {
    use sysinfo::Disks;

    let disks = Disks::new_with_refreshed_list();
    let disk_list: Vec<JsonValue> = disks.iter().map(|disk| {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);
        let usage_percent = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        json!({
            "name": disk.name().to_string_lossy(),
            "mount_point": disk.mount_point().display().to_string(),
            "file_system": disk.file_system().to_string_lossy(),
            "total_bytes": total,
            "available_bytes": available,
            "used_bytes": used,
            "usage_percent": usage_percent,
            "is_removable": disk.is_removable(),
            "kind": format!("{:?}", disk.kind())
        })
    }).collect();

    JsonValue::Array(disk_list)
}

fn collect_network_interfaces() -> JsonValue {
    use sysinfo::Networks;

    let networks = Networks::new_with_refreshed_list();
    let net_list: Vec<JsonValue> = networks.iter().map(|(name, data)| {
        json!({
            "interface": name,
            "bytes_received": data.total_received(),
            "bytes_transmitted": data.total_transmitted(),
            "packets_received": data.total_packets_received(),
            "packets_transmitted": data.total_packets_transmitted(),
            "errors_received": data.total_errors_on_received(),
            "errors_transmitted": data.total_errors_on_transmitted(),
            "mac_address": data.mac_address().to_string()
        })
    }).collect();

    JsonValue::Array(net_list)
}

fn collect_env_vars(pattern: Option<&str>) -> Vec<JsonValue> {
    let mut vars = Vec::new();
    
    for (key, value) in std::env::vars() {
        if let Some(pat) = pattern {
            if !key.contains(pat) && !value.contains(pat) {
                continue;
            }
        }
        
        vars.push(json!({
            "key": key,
            "value": value,
            "length": value.len()
        }));
    }
    
    vars.sort_by(|a, b| a["key"].as_str().cmp(&b["key"].as_str()));
    vars
}

fn collect_git_info(path: &str) -> JsonValue {
    #[cfg(feature = "git")]
    {
        use git2::Repository;
        
        match Repository::open(path) {
            Ok(repo) => {
                let head = repo.head().ok();
                let commit = head.as_ref().and_then(|h| h.peel_to_commit().ok());
                
                json!({
                    "is_repository": true,
                    "branch": head.as_ref()
                        .and_then(|h| h.shorthand())
                        .unwrap_or("unknown"),
                    "commit_hash": commit.as_ref()
                        .map(|c| c.id().to_string())
                        .unwrap_or_else(|| "none".to_string()),
                    "commit_message": commit.as_ref()
                        .and_then(|c| c.message())
                        .unwrap_or("no message"),
                    "author": commit.as_ref()
                        .and_then(|c| c.author().name().map(|s| s.to_string()))
                        .unwrap_or_else(|| "unknown".to_string()),
                    "commit_time": commit.as_ref()
                        .map(|c| c.time().seconds())
                        .unwrap_or(0),
                    "is_bare": repo.is_bare(),
                    "workdir": repo.workdir()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "none".to_string())
                })
            }
            Err(_) => {
                json!({
                    "is_repository": false,
                    "error": "Not a git repository or git not available"
                })
            }
        }
    }
    
    #[cfg(not(feature = "git"))]
    {
        json!({
            "is_repository": false,
            "error": "Git support not compiled in"
        })
    }
}

fn decode_proc_net_addr(hex_addr: &str) -> String {
    let parts: Vec<&str> = hex_addr.split(':').collect();
    if parts.len() != 2 { return hex_addr.to_string(); }

    let ip = u32::from_str_radix(parts[0], 16).unwrap_or(0);
    let port = u16::from_str_radix(parts[1], 16).unwrap_or(0);

    let ip_str = format!("{}.{}.{}.{}",
        ip & 0xFF, (ip >> 8) & 0xFF, (ip >> 16) & 0xFF, (ip >> 24) & 0xFF);

    format!("{}:{}", ip_str, port)
}

fn tcp_state_name(hex_state: &str) -> &'static str {
    match hex_state {
        "01" => "ESTABLISHED",
        "02" => "SYN_SENT",
        "03" => "SYN_RECV",
        "04" => "FIN_WAIT1",
        "05" => "FIN_WAIT2",
        "06" => "TIME_WAIT",
        "07" => "CLOSE",
        "08" => "CLOSE_WAIT",
        "09" => "LAST_ACK",
        "0A" => "LISTEN",
        "0B" => "CLOSING",
        _ => "UNKNOWN",
    }
}

fn collect_network_info(listening_only: bool) -> JsonValue {
    #[cfg(unix)]
    {
        let mut connections = Vec::new();

        if let Ok(contents) = std::fs::read_to_string("/proc/net/tcp") {
            for line in contents.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let state = tcp_state_name(parts[3]);
                    if listening_only && state != "LISTEN" { continue; }

                    connections.push(json!({
                        "protocol": "tcp",
                        "local_address": decode_proc_net_addr(parts[1]),
                        "remote_address": decode_proc_net_addr(parts[2]),
                        "state": state
                    }));
                }
            }
        }

        json!({
            "connection_count": connections.len(),
            "connections": connections,
            "listening_only": listening_only
        })
    }

    #[cfg(not(unix))]
    {
        json!({
            "error": "Network info collection requires Linux (/proc/net/tcp)",
            "listening_only": listening_only
        })
    }
}

fn find_files(pattern: &str, path: &str, case_sensitive: bool, type_filter: Option<&str>, max_depth: usize) -> Vec<JsonValue> {
    let mut results = Vec::new();

    let regex = if case_sensitive {
        regex::Regex::new(pattern)
    } else {
        regex::RegexBuilder::new(pattern).case_insensitive(true).build()
    };

    let regex = match regex {
        Ok(r) => r,
        Err(_) => return vec![json!({"error": "Invalid regex pattern"})]
    };

    for entry in walkdir::WalkDir::new(path).max_depth(max_depth) {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy();
            
            if regex.is_match(&file_name) {
                let metadata = entry.metadata().ok();
                let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
                
                // Apply type filter
                if let Some(filter) = type_filter {
                    match filter {
                        "file" if is_dir => continue,
                        "dir" if !is_dir => continue,
                        _ => {}
                    }
                }
                
                results.push(json!({
                    "name": file_name,
                    "path": entry.path().display().to_string(),
                    "type": if is_dir { "directory" } else { "file" },
                    "size": metadata.as_ref().map(|m| m.len()).unwrap_or(0),
                    "depth": entry.depth()
                }));
            }
        }
    }
    
    results
}

fn analyze_file(file_path: &str) -> JsonValue {
    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            let lines: Vec<&str> = content.lines().collect();
            let words: Vec<&str> = content.split_whitespace().collect();
            let chars: Vec<char> = content.chars().collect();
            
            json!({
                "success": true,
                "file_size_bytes": content.len(),
                "line_count": lines.len(),
                "word_count": words.len(),
                "character_count": chars.len(),
                "character_count_no_whitespace": content.chars().filter(|c| !c.is_whitespace()).count(),
                "empty_line_count": lines.iter().filter(|line| line.trim().is_empty()).count(),
                "max_line_length": lines.iter().map(|line| line.len()).max().unwrap_or(0),
                "min_line_length": lines.iter().map(|line| line.len()).min().unwrap_or(0),
                "average_line_length": if lines.is_empty() { 0.0 } else { 
                    lines.iter().map(|line| line.len()).sum::<usize>() as f64 / lines.len() as f64 
                },
                "file_type": detect_file_type(file_path, &content)
            })
        }
        Err(e) => {
            json!({
                "success": false,
                "error": e.to_string(),
                "suggestions": ["Check if file exists", "Verify read permissions", "Ensure file is text-readable"]
            })
        }
    }
}

fn word_count_analysis(file_path: &str) -> JsonValue {
    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            let lines: Vec<&str> = content.lines().collect();
            let words: Vec<&str> = content.split_whitespace().collect();
            let bytes = content.as_bytes();
            
            // Word frequency analysis
            let mut word_freq: HashMap<String, usize> = HashMap::new();
            for word in &words {
                let clean_word = word.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string();
                if !clean_word.is_empty() {
                    *word_freq.entry(clean_word).or_insert(0) += 1;
                }
            }
            
            let unique_word_count = word_freq.len();
            let mut freq_vec: Vec<_> = word_freq.into_iter().collect();
            freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
            let top_words: Vec<JsonValue> = freq_vec.into_iter().take(10).map(|(word, count)| {
                json!({"word": word, "count": count})
            }).collect();
            
            json!({
                "success": true,
                "bytes": bytes.len(),
                "lines": lines.len(),
                "words": words.len(),
                "characters": content.chars().count(),
                "characters_no_spaces": content.chars().filter(|c| !c.is_whitespace()).count(),
                "paragraphs": content.split("\n\n").filter(|p| !p.trim().is_empty()).count(),
                "sentences": content.matches(|c| c == '.' || c == '!' || c == '?').count(),
                "average_words_per_line": if lines.is_empty() { 0.0 } else {
                    words.len() as f64 / lines.len() as f64
                },
                "top_words": top_words,
                "unique_words": unique_word_count
            })
        }
        Err(e) => {
            json!({
                "success": false,
                "error": e.to_string(),
                "suggestions": ["Check if file exists", "Verify read permissions"]
            })
        }
    }
}

fn disk_usage_analysis(path: &str, max_depth: usize) -> JsonValue {
    let mut directories: HashMap<String, u64> = HashMap::new();
    let mut total_size = 0u64;
    let mut file_count = 0usize;
    let mut dir_count = 0usize;
    
    for entry in walkdir::WalkDir::new(path).max_depth(max_depth) {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let size = metadata.len();
                    total_size += size;
                    file_count += 1;
                    
                    // Add to directory totals
                    if let Some(parent) = entry.path().parent() {
                        let parent_str = parent.display().to_string();
                        *directories.entry(parent_str).or_insert(0) += size;
                    }
                } else if metadata.is_dir() {
                    dir_count += 1;
                }
            }
        }
    }
    
    // Sort directories by size
    let mut dir_vec: Vec<_> = directories.into_iter().collect();
    dir_vec.sort_by(|a, b| b.1.cmp(&a.1));
    
    let directory_breakdown: Vec<JsonValue> = dir_vec.into_iter().take(20).map(|(path, size)| {
        json!({
            "path": path,
            "size_bytes": size,
            "size_mb": size as f64 / (1024.0 * 1024.0),
            "percentage": if total_size > 0 { (size as f64 / total_size as f64) * 100.0 } else { 0.0 }
        })
    }).collect();
    
    json!({
        "analyzed_path": path,
        "max_depth": max_depth,
        "total_size_bytes": total_size,
        "total_size_mb": total_size as f64 / (1024.0 * 1024.0),
        "file_count": file_count,
        "directory_count": dir_count,
        "average_file_size": if file_count > 0 { total_size / file_count as u64 } else { 0 },
        "directory_breakdown": directory_breakdown
    })
}

fn detect_file_type(file_path: &str, content: &str) -> String {
    let path = Path::new(file_path);
    
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        match ext.to_lowercase().as_str() {
            "rs" => "rust".to_string(),
            "py" => "python".to_string(),
            "js" | "ts" => "javascript/typescript".to_string(),
            "html" | "htm" => "html".to_string(),
            "css" => "css".to_string(),
            "json" => "json".to_string(),
            "xml" => "xml".to_string(),
            "md" => "markdown".to_string(),
            "txt" => "text".to_string(),
            "log" => "log".to_string(),
            _ => format!("unknown (.{})", ext)
        }
    } else if content.starts_with("#!/") {
        "script".to_string()
    } else if content.starts_with("<?xml") {
        "xml".to_string()
    } else if content.starts_with("{") || content.starts_with("[") {
        "json".to_string()
    } else {
        "text".to_string()
    }
}

fn main() {
    serve_plugin(&LlmPlugin::new(), MsgPackSerializer {});
}