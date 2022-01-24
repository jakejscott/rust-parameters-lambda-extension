use anyhow::Result;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::{spawn, task::JoinHandle};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub args: String,
    pub items: Vec<ParameterItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterItem {
    pub name: String,
    pub value: String,
}

pub async fn ssm_get_parameter(
    ssm: &aws_sdk_ssm::Client,
    name: String,
    args: String,
) -> Result<Parameter> {
    let response = ssm
        .get_parameter()
        .name(args.replace("ssm_parameter:", ""))
        .with_decryption(true)
        .send()
        .await?;

    let parameter = response.parameter.unwrap();

    Ok(Parameter {
        name: name.to_owned(),
        args: args.to_owned(),
        items: vec![ParameterItem {
            name: parameter.name.unwrap(),
            value: parameter.value.unwrap(),
        }],
    })
}

pub async fn ssm_get_parameters_by_path(
    ssm: &aws_sdk_ssm::Client,
    name: String,
    args: String,
) -> Result<Parameter> {
    let mut items: Vec<ParameterItem> = Vec::new();

    let mut token: Option<String> = None;

    loop {
        let response = ssm
            .get_parameters_by_path()
            .path(args.replace("ssm_parameters:", ""))
            .recursive(true)
            .with_decryption(true)
            .set_next_token(token.clone())
            .send()
            .await?;

        for parameters in response.parameters {
            for parameter in parameters {
                items.push(ParameterItem {
                    name: parameter.name.unwrap(),
                    value: parameter.value.unwrap(),
                });
            }
        }

        if response.next_token == None {
            break;
        }

        token = response.next_token;
    }

    Ok(Parameter {
        name: name.to_owned(),
        args: args.to_owned(),
        items: items,
    })
}

pub async fn fetch_parameters(
    vars: HashMap<String, String>,
    ssm: &aws_sdk_ssm::Client,
) -> Vec<Parameter> {
    let mut results: Vec<Parameter> = Vec::new();

    let mut handles: Vec<JoinHandle<Result<Parameter>>> = Vec::new();

    for (name, args) in vars {
        if args.starts_with("ssm_parameter:") {
            let ssm_clone = ssm.clone();
            handles.push(spawn(async move {
                ssm_get_parameter(&ssm_clone, name, args).await
            }));
        } else if args.starts_with("ssm_parameters:") {
            let ssm_clone = ssm.clone();
            handles.push(spawn(async move {
                ssm_get_parameters_by_path(&ssm_clone, name, args).await
            }));
        }
    }

    let tasks = join_all(handles).await;

    for task in tasks {
        match task {
            Ok(result) => match result {
                Ok(parameter) => {
                    results.push(parameter);
                }
                Err(_) => todo!(),
            },
            Err(_) => todo!(),
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use aws_sdk_ssm::model::ParameterType;

    #[tokio::test]
    async fn should_parse() -> Result<()> {
        let config = aws_config::load_from_env().await;
        let ssm = aws_sdk_ssm::Client::new(&config);

        ssm.put_parameter()
            .name("/my/parameter".to_owned())
            .value("my-parameter".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;

        ssm.put_parameter()
            .name("/my/path/prefix/value/1".to_owned())
            .value("value-1".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;

        ssm.put_parameter()
            .name("/my/path/prefix/value/2".to_owned())
            .value("value-2".to_owned())
            .r#type(ParameterType::SecureString)
            .overwrite(true)
            .send()
            .await?;

        let vars: HashMap<String, String> = HashMap::from([
            (
                "MY_PARAMETER".to_string(),
                "ssm_parameter:/my/parameter".to_string(),
            ),
            (
                "MY_PARAMETERS".to_string(),
                "ssm_parameters:/my/path/prefix".to_string(),
            ),
        ]);

        let results = fetch_parameters(vars, &ssm).await;
        println!("Parameters {:#?}", results);

        Ok(())
    }
}
