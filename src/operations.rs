use http::Method;
use serde_json::Value;

use crate::error::CliError;

#[derive(Debug, Clone)]
pub struct OperationSpec {
    pub name: &'static str,
    pub method: Method,
    pub path: &'static str,
    pub has_body: bool,
    pub body_required: bool,
    pub success_checks: &'static [SuccessCheck],
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SuccessCheck {
    FieldTrue(&'static str),
    FieldAbsentOrEmpty(&'static str),
}

impl OperationSpec {
    pub fn evaluate(self, value: &Value) -> Result<(), CliError> {
        for check in self.success_checks {
            match check {
                SuccessCheck::FieldTrue(field) => {
                    if value.get(field).and_then(Value::as_bool) != Some(true) {
                        return Err(CliError::SemanticFailure(render_semantic_failure(value)));
                    }
                }
                SuccessCheck::FieldAbsentOrEmpty(field) => {
                    let Some(candidate) = value.get(field) else {
                        continue;
                    };

                    let is_empty = match candidate {
                        Value::Null => true,
                        Value::Array(items) => items.is_empty(),
                        Value::Object(map) => map.is_empty(),
                        Value::String(text) => text.trim().is_empty(),
                        _ => false,
                    };

                    if !is_empty {
                        return Err(CliError::SemanticFailure(render_semantic_failure(value)));
                    }
                }
            }
        }

        Ok(())
    }
}

fn render_semantic_failure(value: &Value) -> String {
    value
        .get("errors")
        .map(render_compact)
        .unwrap_or_else(|| render_compact(value))
}

fn render_compact(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(text) => text.clone(),
        _ => serde_json::to_string(value).unwrap_or_else(|_| "<unprintable json>".to_string()),
    }
}

const NO_CHECKS: &[SuccessCheck] = &[];
const SUCCESS_TRUE: &[SuccessCheck] = &[SuccessCheck::FieldTrue("success")];
const ERRORS_EMPTY: &[SuccessCheck] = &[SuccessCheck::FieldAbsentOrEmpty("errors")];

pub const GET_CURRENT_USER: OperationSpec = OperationSpec {
    name: "get_current_user",
    method: Method::GET,
    path: "/get_current_user",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const GET_USER: OperationSpec = OperationSpec {
    name: "get_user",
    method: Method::GET,
    path: "/get_user/{id}",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const UPDATE_USER: OperationSpec = OperationSpec {
    name: "update_user",
    method: Method::POST,
    path: "/update_user/{id}",
    has_body: true,
    body_required: true,
    success_checks: NO_CHECKS,
};

pub const GET_GROUPS: OperationSpec = OperationSpec {
    name: "get_groups",
    method: Method::GET,
    path: "/get_groups",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const GET_GROUP: OperationSpec = OperationSpec {
    name: "get_group",
    method: Method::GET,
    path: "/get_group/{id}",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const CREATE_GROUP: OperationSpec = OperationSpec {
    name: "create_group",
    method: Method::POST,
    path: "/create_group",
    has_body: true,
    body_required: true,
    success_checks: ERRORS_EMPTY,
};

pub const DELETE_GROUP: OperationSpec = OperationSpec {
    name: "delete_group",
    method: Method::POST,
    path: "/delete_group/{id}",
    has_body: false,
    body_required: false,
    success_checks: SUCCESS_TRUE,
};

pub const UNDELETE_GROUP: OperationSpec = OperationSpec {
    name: "undelete_group",
    method: Method::POST,
    path: "/undelete_group/{id}",
    has_body: false,
    body_required: false,
    success_checks: SUCCESS_TRUE,
};

pub const ADD_USER_TO_GROUP: OperationSpec = OperationSpec {
    name: "add_user_to_group",
    method: Method::POST,
    path: "/add_user_to_group",
    has_body: true,
    body_required: true,
    success_checks: ERRORS_EMPTY,
};

pub const REMOVE_USER_FROM_GROUP: OperationSpec = OperationSpec {
    name: "remove_user_from_group",
    method: Method::POST,
    path: "/remove_user_from_group",
    has_body: true,
    body_required: true,
    success_checks: SUCCESS_TRUE,
};

pub const GET_FRIENDS: OperationSpec = OperationSpec {
    name: "get_friends",
    method: Method::GET,
    path: "/get_friends",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const GET_FRIEND: OperationSpec = OperationSpec {
    name: "get_friend",
    method: Method::GET,
    path: "/get_friend/{id}",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const CREATE_FRIEND: OperationSpec = OperationSpec {
    name: "create_friend",
    method: Method::POST,
    path: "/create_friend",
    has_body: true,
    body_required: true,
    success_checks: NO_CHECKS,
};

pub const CREATE_FRIENDS: OperationSpec = OperationSpec {
    name: "create_friends",
    method: Method::POST,
    path: "/create_friends",
    has_body: true,
    body_required: true,
    success_checks: ERRORS_EMPTY,
};

pub const DELETE_FRIEND: OperationSpec = OperationSpec {
    name: "delete_friend",
    method: Method::POST,
    path: "/delete_friend/{id}",
    has_body: false,
    body_required: false,
    success_checks: SUCCESS_TRUE,
};

pub const GET_CURRENCIES: OperationSpec = OperationSpec {
    name: "get_currencies",
    method: Method::GET,
    path: "/get_currencies",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const GET_EXPENSE: OperationSpec = OperationSpec {
    name: "get_expense",
    method: Method::GET,
    path: "/get_expense/{id}",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const GET_EXPENSES: OperationSpec = OperationSpec {
    name: "get_expenses",
    method: Method::GET,
    path: "/get_expenses",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const CREATE_EXPENSE: OperationSpec = OperationSpec {
    name: "create_expense",
    method: Method::POST,
    path: "/create_expense",
    has_body: true,
    body_required: true,
    success_checks: ERRORS_EMPTY,
};

pub const UPDATE_EXPENSE: OperationSpec = OperationSpec {
    name: "update_expense",
    method: Method::POST,
    path: "/update_expense/{id}",
    has_body: true,
    body_required: true,
    success_checks: ERRORS_EMPTY,
};

pub const DELETE_EXPENSE: OperationSpec = OperationSpec {
    name: "delete_expense",
    method: Method::POST,
    path: "/delete_expense/{id}",
    has_body: false,
    body_required: false,
    success_checks: SUCCESS_TRUE,
};

pub const UNDELETE_EXPENSE: OperationSpec = OperationSpec {
    name: "undelete_expense",
    method: Method::POST,
    path: "/undelete_expense/{id}",
    has_body: false,
    body_required: false,
    success_checks: SUCCESS_TRUE,
};

pub const GET_COMMENTS: OperationSpec = OperationSpec {
    name: "get_comments",
    method: Method::GET,
    path: "/get_comments",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const CREATE_COMMENT: OperationSpec = OperationSpec {
    name: "create_comment",
    method: Method::POST,
    path: "/create_comment",
    has_body: true,
    body_required: true,
    success_checks: NO_CHECKS,
};

pub const DELETE_COMMENT: OperationSpec = OperationSpec {
    name: "delete_comment",
    method: Method::POST,
    path: "/delete_comment/{id}",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const GET_NOTIFICATIONS: OperationSpec = OperationSpec {
    name: "get_notifications",
    method: Method::GET,
    path: "/get_notifications",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const GET_CATEGORIES: OperationSpec = OperationSpec {
    name: "get_categories",
    method: Method::GET,
    path: "/get_categories",
    has_body: false,
    body_required: false,
    success_checks: NO_CHECKS,
};

pub const ALL_OPERATIONS: &[OperationSpec] = &[
    GET_CURRENT_USER,
    GET_USER,
    UPDATE_USER,
    GET_GROUPS,
    GET_GROUP,
    CREATE_GROUP,
    DELETE_GROUP,
    UNDELETE_GROUP,
    ADD_USER_TO_GROUP,
    REMOVE_USER_FROM_GROUP,
    GET_FRIENDS,
    GET_FRIEND,
    CREATE_FRIEND,
    CREATE_FRIENDS,
    DELETE_FRIEND,
    GET_CURRENCIES,
    GET_EXPENSE,
    GET_EXPENSES,
    CREATE_EXPENSE,
    UPDATE_EXPENSE,
    DELETE_EXPENSE,
    UNDELETE_EXPENSE,
    GET_COMMENTS,
    CREATE_COMMENT,
    DELETE_COMMENT,
    GET_NOTIFICATIONS,
    GET_CATEGORIES,
];

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        ADD_USER_TO_GROUP, ALL_OPERATIONS, CREATE_EXPENSE, DELETE_EXPENSE, DELETE_FRIEND,
        UNDELETE_GROUP,
    };

    #[test]
    fn operation_count_matches_mcp_surface() {
        assert_eq!(ALL_OPERATIONS.len(), 27);
    }

    #[test]
    fn success_true_operations_require_true() {
        DELETE_FRIEND
            .evaluate(&json!({"success": true, "errors": {}}))
            .expect("success=true should pass");

        let error = DELETE_EXPENSE
            .evaluate(&json!({"success": false, "errors": {"expense": ["missing"]}}))
            .expect_err("success=false should fail");
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn errors_must_be_empty_when_required() {
        CREATE_EXPENSE
            .evaluate(&json!({"expenses": [], "errors": {}}))
            .expect("empty errors should pass");

        ADD_USER_TO_GROUP
            .evaluate(&json!({"success": false, "errors": {"base": ["blocked"]}}))
            .expect_err("non-empty errors should fail");
    }

    #[test]
    fn absent_errors_field_is_treated_as_success() {
        UNDELETE_GROUP
            .evaluate(&json!({"success": true}))
            .expect("missing errors field should not fail");
    }
}
