use serde_json::json;

mod common;

use common::*;

#[tokio::test]
async fn levels_examples() -> anyhow::Result<()> {
    assert_eval(
        r#"
pattern test = {
    #[advice("advice")]
    advice: false,
    #[warning("warning")]
    warning: false,
    // defaults to #[error]
    #[explain("error")]
    error: false,
}
"#,
        json!({
            "advice": true,
            "warning": true,
            "error": true,
        }),
        json!({
            "name": "test::test",
            "severity": "error",
            "reason": "because not all fields were satisfied",
            "rationale": [
                {
                    "name": "field:advice",
                    "severity": "advice",
                    "reason": "advice",
                    "rationale": [{
                        "severity": "advice",
                        "reason": "advice",
                    }]
                },
                {
                    "name": "field:error",
                    "severity": "error",
                    "reason": "error",
                    "rationale": [{
                        "severity": "error",
                        "reason": "error",
                    }]
                },
                {
                    "name": "field:warning",
                    "severity": "warning",
                    "reason": "warning",
                    "rationale": [{
                        "severity": "warning",
                        "reason": "warning",
                    }]
                },
            ]
        }),
    )
    .await;

    Ok(())
}

#[tokio::test]
async fn levels_default_to_highest() -> anyhow::Result<()> {
    assert_eval(
        r#"
pattern test = {
    #[advice("advice")]
    advice: false,
    #[warning("warning")]
    warning: false,
    // defaults to #[error]
    #[explain("error")]
    error: false,
}
"#,
        json!({
            "advice": true,
            "warning": false,
            "error": false,
        }),
        json!({
            "name": "test::test",
            "severity": "advice",
            "reason": "because not all fields were satisfied",
            "rationale": [
                {
                    "name": "field:advice",
                    "severity": "advice",
                    "reason": "advice",
                    "rationale": [{
                        "severity": "advice",
                        "reason": "advice",
                    }]
                },
                {
                    "name": "field:error",
                    "rationale": [{}]
                },
                {
                    "name": "field:warning",
                    "rationale": [{}]
                },
            ]
        }),
    )
    .await;

    Ok(())
}