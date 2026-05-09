#[macro_export]
macro_rules! wiki {
    (
        $name:expr,
        $module:expr,
        $signature:expr,
        $description:expr
    ) => {
        $crate::wiki::metadata::MethodDoc {
            name: $name.to_string(),
            module: $module.to_string(),
            signature: $signature.to_string(),
            description: $description.to_string(),
            parameters: vec![],
            returns: None,
            examples: vec![],
            since_version: None,
            deprecated: false,
        }
    };
    (
        $name:expr,
        $module:expr,
        $signature:expr,
        $description:expr,
        params: [$($param_name:expr, $param_type:expr, $param_desc:expr),*],
        returns: $return_type:expr
    ) => {
        {
            let params = vec![
                $($crate::wiki::metadata::ParamDoc {
                    name: $param_name.to_string(),
                    param_type: $param_type.to_string(),
                    description: $param_desc.to_string(),
                }),*
            ];
            $crate::wiki::metadata::MethodDoc {
                name: $name.to_string(),
                module: $module.to_string(),
                signature: $signature.to_string(),
                description: $description.to_string(),
                parameters: params,
                returns: Some($return_type.to_string()),
                examples: vec![],
                since_version: None,
                deprecated: false,
            }
        }
    };
    (
        $name:expr,
        $module:expr,
        $signature:expr,
        $description:expr,
        params: [$($param_name:expr, $param_type:expr, $param_desc:expr),*],
        returns: $return_type:expr,
        examples: [$($example:expr),*],
        since: $version:expr
    ) => {
        {
            let params = vec![
                $($crate::wiki::metadata::ParamDoc {
                    name: $param_name.to_string(),
                    param_type: $param_type.to_string(),
                    description: $param_desc.to_string(),
                }),*
            ];
            let examples = vec![$($example.to_string()),*];
            $crate::wiki::metadata::MethodDoc {
                name: $name.to_string(),
                module: $module.to_string(),
                signature: $signature.to_string(),
                description: $description.to_string(),
                parameters: params,
                returns: Some($return_type.to_string()),
                examples,
                since_version: Some($version.to_string()),
                deprecated: false,
            }
        }
    };
    (
        $name:expr,
        $module:expr,
        $signature:expr,
        $description:expr,
        deprecated
    ) => {
        {
            let mut method = $crate::wiki::metadata::MethodDoc {
                name: $name.to_string(),
                module: $module.to_string(),
                signature: $signature.to_string(),
                description: $description.to_string(),
                parameters: vec![],
                returns: None,
                examples: vec![],
                since_version: None,
                deprecated: false,
            };
            method.deprecated = true;
            method
        }
    };
}


