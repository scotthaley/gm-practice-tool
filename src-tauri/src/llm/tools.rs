use crate::llm::types::{ToolDefinition, ToolDefinitionWrapper};
use rand::Rng;
use serde_json::json;

pub fn get_player_tools() -> Vec<ToolDefinitionWrapper> {
    vec![
        ToolDefinitionWrapper {
            tool_type: "function".to_string(),
            function: ToolDefinition {
                name: "roll_dice".to_string(),
                description: "Roll dice using standard notation (e.g., '2d6+3', '1d20', '4d6'). Use this when your character needs to make a check, attack, or any action that requires a dice roll.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "notation": {
                            "type": "string",
                            "description": "Dice notation like '2d6+3', '1d20', '4d6-1'"
                        },
                        "reason": {
                            "type": "string",
                            "description": "Why the roll is being made (e.g., 'perception check', 'attack roll')"
                        }
                    },
                    "required": ["notation", "reason"]
                }),
            },
        },
        ToolDefinitionWrapper {
            tool_type: "function".to_string(),
            function: ToolDefinition {
                name: "store_memory".to_string(),
                description: "Store an important memory or observation that your character would remember. Use this for significant events, NPC names, plot points, or things your character notices.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "content": {
                            "type": "string",
                            "description": "The memory content to store"
                        },
                        "memory_type": {
                            "type": "string",
                            "enum": ["observation", "emotion", "plan", "knowledge", "relationship"],
                            "description": "Type of memory"
                        },
                        "importance": {
                            "type": "string",
                            "enum": ["low", "medium", "high", "critical"],
                            "description": "How important this memory is: low (trivial detail), medium (useful info), high (significant event), critical (game-changing)"
                        }
                    },
                    "required": ["content", "memory_type", "importance"]
                }),
            },
        },
        ToolDefinitionWrapper {
            tool_type: "function".to_string(),
            function: ToolDefinition {
                name: "add_campaign_log".to_string(),
                description: "Add an entry to the campaign log. Use this to record significant actions or events that happen during the scene.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "summary": {
                            "type": "string",
                            "description": "Brief summary of what happened"
                        },
                        "entry_type": {
                            "type": "string",
                            "enum": ["action", "dialogue", "combat", "discovery", "social"],
                            "description": "Type of log entry"
                        }
                    },
                    "required": ["summary", "entry_type"]
                }),
            },
        },
        ToolDefinitionWrapper {
            tool_type: "function".to_string(),
            function: ToolDefinition {
                name: "update_rules".to_string(),
                description: "Append a rule clarification or update to the campaign's ruleset. Use this when the GM explains, clarifies, or modifies a game rule, or when a house rule is established during play.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "clarification": {
                            "type": "string",
                            "description": "The rule clarification, explanation, or house rule to append to the ruleset"
                        }
                    },
                    "required": ["clarification"]
                }),
            },
        },
        ToolDefinitionWrapper {
            tool_type: "function".to_string(),
            function: ToolDefinition {
                name: "recall_memory".to_string(),
                description: "Try to recall a memory about a specific topic. Use this when your character is trying to remember something relevant.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "What to try to remember"
                        }
                    },
                    "required": ["query"]
                }),
            },
        },
    ]
}

#[derive(Debug)]
pub struct DiceResult {
    pub notation: String,
    pub rolls: Vec<u32>,
    pub modifier: i32,
    pub total: i32,
}

pub fn roll_dice(notation: &str) -> Result<DiceResult, String> {
    let notation = notation.trim().to_lowercase();

    // Parse notation like "2d6+3", "1d20", "4d6-1"
    let (dice_part, modifier) = if let Some(pos) = notation.find('+') {
        let (d, m) = notation.split_at(pos);
        (d.to_string(), m[1..].parse::<i32>().unwrap_or(0))
    } else if let Some(pos) = notation.rfind('-') {
        if pos == 0 {
            return Err("Invalid dice notation".to_string());
        }
        let (d, m) = notation.split_at(pos);
        (d.to_string(), -(m[1..].parse::<i32>().unwrap_or(0)))
    } else {
        (notation.clone(), 0)
    };

    let parts: Vec<&str> = dice_part.split('d').collect();
    if parts.len() != 2 {
        return Err("Invalid dice notation. Use format like '2d6+3'".to_string());
    }

    let num_dice: u32 = parts[0].parse().unwrap_or(1);
    let num_sides: u32 = parts[1].parse().map_err(|_| "Invalid number of sides".to_string())?;

    if num_dice == 0 || num_dice > 100 || num_sides == 0 || num_sides > 1000 {
        return Err("Dice values out of reasonable range".to_string());
    }

    let mut rng = rand::thread_rng();
    let rolls: Vec<u32> = (0..num_dice).map(|_| rng.gen_range(1..=num_sides)).collect();
    let sum: u32 = rolls.iter().sum();
    let total = sum as i32 + modifier;

    Ok(DiceResult {
        notation: notation.to_string(),
        rolls,
        modifier,
        total,
    })
}
