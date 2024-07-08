use spin_sdk::{
    http::{IntoResponse, Request, Response},
    http_component,
    key_value::Store,
    llm::{infer, InferencingModel},
};


use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct SampleResponse {
    pub input: String,
}


const PROMPT_TEMPLATE: &str = r#"<<SYS>>
You are an expert programmer that helps to write code based on the user request, with concise explanations. Don't be too verbose.// <</SYS>>

"#;

const PROMPT_KEY : &str = "prompt_history";


fn place_prompt_key_for_first_time(){
    let store = Store::open_default().unwrap();
    // set only for the first time if key doesn't exist
    let encoded_prompt: Option<Vec<u8>> = store.get(PROMPT_KEY).unwrap_or_default(); 

    match encoded_prompt {
        Some(_) => {
            println!("Prompt key already exists");
        },
        None => {
            match store.set(PROMPT_KEY, PROMPT_TEMPLATE.as_bytes()) {
                Ok(_) => println!("Data saved successfully."),
                Err(e) => println!("Failed to save data: {}", e),
            }
        }
    }

}

#[http_component]
fn handle_code_pls(req: Request) -> anyhow::Result<impl IntoResponse> {
  
    let store = Store::open_default()?;
    
    place_prompt_key_for_first_time();

    let encoded_prompt: Option<Vec<u8>> = store.get(PROMPT_KEY).unwrap_or_default();

    // println!("Encoded prompt {:?}", encoded_prompt);
    let mut prompt_history = String::new();

    if let Some(mut vec) = encoded_prompt {
        let slice: &mut [u8] = &mut vec;

        let s = match std::str::from_utf8(slice) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        prompt_history = s.to_string();
    }else{

        println!("Not able to fetch data :( {:?}", encoded_prompt);
    }
    // println!("Prompt history {:?}", prompt_history);

    let body = std::str::from_utf8(req.body()).unwrap();
    println!("Request {:?}", body);
    let request: SampleResponse= match serde_json::from_slice(req.body()){

        Ok(body) => body,
        Err(error) => {
            println!("Error in parsing the request body {:?}", error);
            return anyhow::Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body("Error while parsing reqeust")
            .build());
        }
    };
    // println!("Request {:?}", request);
    let model = InferencingModel::CodellamaInstruct;
    prompt_history += request.input.as_str();


    // let encoded_input = 
    match store.set(PROMPT_KEY, &prompt_history.clone().into_bytes()) {
        Ok(_) => println!("Data saved successfully."),
        Err(e) => println!("Failed to save data: {}", e),
    }

    // conn.set("prompt_history", );
    let result = infer(model, &prompt_history);
    // println!("Model result {:?}", result);
    let result_text = match result {
        Ok(r) => r.text,
        Err(_) => "Error in LLM".to_string(),

    };
    println!("Result text {:?}", result_text);

    anyhow::Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(result_text)
        .build())
}


