//! Spec-driven development MCP tool handlers

use crate::core::spec::{DesignSpec, RequirementSpec, Specification, TaskPlan};
use crate::mcp::service::VibeTicketService;
use crate::storage::TicketRepository;
use rmcp::model::Tool;
use serde::Deserialize;
use serde_json::{json, Value};
use std::borrow::Cow;
use std::sync::Arc;

/// Register all spec-driven development tools
pub fn register_tools() -> Vec<Tool> {
    vec![
        // Add spec tool
        Tool {
            name: Cow::Borrowed("vibe-ticket.spec.add"),
            description: Some(Cow::Borrowed("Add specifications to a ticket")),
            input_schema: Arc::new(json!({
                "type": "object",
                "properties": {
                    "ticket": {
                        "type": "string",
                        "description": "Ticket ID or slug"
                    },
                    "requirements": {
                        "type": "object",
                        "description": "Requirements specification",
                        "properties": {
                            "functional": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "Functional requirements"
                            },
                            "non_functional": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "Non-functional requirements"
                            },
                            "constraints": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "Constraints"
                            },
                            "assumptions": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "Assumptions"
                            }
                        }
                    },
                    "design": {
                        "type": "object",
                        "description": "Design specification",
                        "properties": {
                            "overview": {
                                "type": "string",
                                "description": "Design overview"
                            },
                            "components": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "System components"
                            },
                            "interfaces": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "Interface definitions"
                            },
                            "data_flow": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "Data flow descriptions"
                            }
                        }
                    },
                    "tasks": {
                        "type": "array",
                        "description": "Task plan",
                        "items": {
                            "type": "object",
                            "properties": {
                                "phase": {
                                    "type": "string",
                                    "description": "Task phase"
                                },
                                "name": {
                                    "type": "string",
                                    "description": "Task name"
                                },
                                "description": {
                                    "type": "string",
                                    "description": "Task description"
                                },
                                "estimated_hours": {
                                    "type": "number",
                                    "description": "Estimated hours"
                                }
                            }
                        }
                    }
                },
                "required": ["ticket"]
            })),
            annotations: None,
        },
        // Update spec tool
        Tool {
            name: Cow::Borrowed("vibe-ticket.spec.update"),
            description: Some(Cow::Borrowed("Update specifications for a ticket")),
            input_schema: Arc::new(json!({
                "type": "object",
                "properties": {
                    "ticket": {
                        "type": "string",
                        "description": "Ticket ID or slug"
                    },
                    "requirements": {
                        "type": "object",
                        "description": "Updated requirements specification"
                    },
                    "design": {
                        "type": "object",
                        "description": "Updated design specification"
                    },
                    "tasks": {
                        "type": "array",
                        "description": "Updated task plan"
                    }
                },
                "required": ["ticket"]
            })),
            annotations: None,
        },
        // Check spec tool
        Tool {
            name: Cow::Borrowed("vibe-ticket.spec.check"),
            description: Some(Cow::Borrowed("Check specification status for a ticket")),
            input_schema: Arc::new(json!({
                "type": "object",
                "properties": {
                    "ticket": {
                        "type": "string",
                        "description": "Ticket ID or slug"
                    }
                },
                "required": ["ticket"]
            })),
            annotations: None,
        },
    ]
}

/// Handle adding specifications
pub async fn handle_add(service: &VibeTicketService, arguments: Value) -> Result<Value, String> {
    #[derive(Deserialize)]
    struct Args {
        ticket: String,
        requirements: Option<RequirementsInput>,
        design: Option<DesignInput>,
        tasks: Option<Vec<TaskInput>>,
    }

    #[derive(Deserialize)]
    struct RequirementsInput {
        functional: Option<Vec<String>>,
        non_functional: Option<Vec<String>>,
        constraints: Option<Vec<String>>,
        assumptions: Option<Vec<String>>,
    }

    #[derive(Deserialize)]
    struct DesignInput {
        overview: Option<String>,
        components: Option<Vec<String>>,
        interfaces: Option<Vec<String>>,
        data_flow: Option<Vec<String>>,
    }

    #[derive(Deserialize)]
    struct TaskInput {
        phase: String,
        name: String,
        description: Option<String>,
        estimated_hours: Option<f32>,
    }

    let args: Args = serde_json::from_value(arguments)
        .map_err(|e| format!("Invalid arguments: {}", e))?;

    let ticket_id = crate::mcp::handlers::tickets::resolve_ticket_ref(service, &args.ticket).await?;
    let mut ticket = service.storage.load(&ticket_id)
        .map_err(|e| format!("Failed to load ticket: {}", e))?;

    // Initialize specification if not present
    if !ticket.metadata.contains_key("specification") {
        let spec = Specification::default();
        let spec_json = serde_json::to_string(&spec)
            .map_err(|e| format!("Failed to serialize specification: {}", e))?;
        ticket.metadata.insert("specification".to_string(), spec_json);
    }

    // Parse existing specification
    let mut spec: Specification = ticket.metadata.get("specification")
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();

    let mut updates = Vec::new();

    // Update requirements if provided
    if let Some(req_input) = args.requirements {
        if let Some(functional) = req_input.functional {
            spec.requirements.functional = functional;
            updates.push("requirements.functional");
        }
        if let Some(non_functional) = req_input.non_functional {
            spec.requirements.non_functional = non_functional;
            updates.push("requirements.non_functional");
        }
        if let Some(constraints) = req_input.constraints {
            spec.requirements.constraints = constraints;
            updates.push("requirements.constraints");
        }
        if let Some(assumptions) = req_input.assumptions {
            spec.requirements.assumptions = assumptions;
            updates.push("requirements.assumptions");
        }
        spec.requirements.updated_at = chrono::Utc::now();
    }

    // Update design if provided
    if let Some(design_input) = args.design {
        if let Some(overview) = design_input.overview {
            spec.design.overview = overview;
            updates.push("design.overview");
        }
        if let Some(components) = design_input.components {
            spec.design.components = components;
            updates.push("design.components");
        }
        if let Some(interfaces) = design_input.interfaces {
            spec.design.interfaces = interfaces;
            updates.push("design.interfaces");
        }
        if let Some(data_flow) = design_input.data_flow {
            spec.design.data_flow = data_flow;
            updates.push("design.data_flow");
        }
        spec.design.updated_at = chrono::Utc::now();
    }

    // Update tasks if provided
    if let Some(task_inputs) = args.tasks {
        spec.tasks.tasks = task_inputs.into_iter().map(|t| crate::core::spec::SpecTask {
            phase: t.phase,
            name: t.name,
            description: t.description.unwrap_or_default(),
            estimated_hours: t.estimated_hours,
            actual_hours: None,
            completed: false,
        }).collect();
        spec.tasks.updated_at = chrono::Utc::now();
        updates.push("tasks");
    }

    if updates.is_empty() {
        return Ok(json!({
            "status": "unchanged",
            "message": "No specification updates provided"
        }));
    }

    // Save updated specification
    let spec_json = serde_json::to_string(&spec)
        .map_err(|e| format!("Failed to serialize specification: {}", e))?;
    ticket.metadata.insert("specification".to_string(), spec_json);

    service.storage.save(&ticket)
        .map_err(|e| format!("Failed to save ticket: {}", e))?;

    Ok(json!({
        "status": "added",
        "ticket_id": ticket.id.to_string(),
        "ticket_slug": ticket.slug,
        "updated": updates
    }))
}

/// Handle updating specifications
pub async fn handle_update(service: &VibeTicketService, arguments: Value) -> Result<Value, String> {
    // Update uses the same logic as add
    handle_add(service, arguments).await
}

/// Handle checking specification status
pub async fn handle_check(service: &VibeTicketService, arguments: Value) -> Result<Value, String> {
    #[derive(Deserialize)]
    struct Args {
        ticket: String,
    }

    let args: Args = serde_json::from_value(arguments)
        .map_err(|e| format!("Invalid arguments: {}", e))?;

    let ticket_id = crate::mcp::handlers::tickets::resolve_ticket_ref(service, &args.ticket).await?;
    let ticket = service.storage.load(&ticket_id)
        .map_err(|e| format!("Failed to load ticket: {}", e))?;

    if let Some(spec_json) = ticket.metadata.get("specification") {
        let spec: Specification = serde_json::from_str(spec_json)
            .map_err(|e| format!("Failed to parse specification: {}", e))?;

        // Calculate completion stats
        let req_complete = !spec.requirements.functional.is_empty() || !spec.requirements.non_functional.is_empty();
        let design_complete = !spec.design.overview.is_empty() || !spec.design.components.is_empty();
        let tasks_total = spec.tasks.tasks.len();
        let tasks_completed = spec.tasks.tasks.iter().filter(|t| t.completed).count();
        let total_estimated_hours: f32 = spec.tasks.tasks.iter()
            .filter_map(|t| t.estimated_hours)
            .sum();

        Ok(json!({
            "ticket_id": ticket.id.to_string(),
            "ticket_slug": ticket.slug,
            "specification": {
                "requirements": {
                    "status": if req_complete { "defined" } else { "empty" },
                    "functional_count": spec.requirements.functional.len(),
                    "non_functional_count": spec.requirements.non_functional.len(),
                    "constraints_count": spec.requirements.constraints.len(),
                    "assumptions_count": spec.requirements.assumptions.len(),
                    "updated_at": spec.requirements.updated_at.to_rfc3339()
                },
                "design": {
                    "status": if design_complete { "defined" } else { "empty" },
                    "has_overview": !spec.design.overview.is_empty(),
                    "components_count": spec.design.components.len(),
                    "interfaces_count": spec.design.interfaces.len(),
                    "data_flow_count": spec.design.data_flow.len(),
                    "updated_at": spec.design.updated_at.to_rfc3339()
                },
                "tasks": {
                    "total": tasks_total,
                    "completed": tasks_completed,
                    "percentage": if tasks_total > 0 { (tasks_completed * 100) / tasks_total } else { 0 },
                    "estimated_hours": total_estimated_hours,
                    "updated_at": spec.tasks.updated_at.to_rfc3339()
                }
            }
        }))
    } else {
        Ok(json!({
            "ticket_id": ticket.id.to_string(),
            "ticket_slug": ticket.slug,
            "specification": {
                "status": "not_defined",
                "message": "No specification found for this ticket"
            }
        }))
    }
}