use calamine::{open_workbook, Reader, Xlsx};
use sajko::{
    factors::{Question1, Questionaire},
    gpt_models::{GptResponse, Message, Prompt},
};

const SKIP_CHAT_GPT: bool = true;

#[tokio::main]
async fn main() {
    println!("Hello, psycho!");
    let mut questions = vec![];

    // read questionaire Excel file with columns: ordinal number, score, answer, difference, code, question. The resulting array should be of type Vec<(i32, i32, i32, String, String, String)> (example: [1, 1, 1, "S", "n1", "Nisam od onih koji stalno brinu."] )
    let mut excel: Xlsx<_> = open_workbook("questionnaire/neo-pi-r-korekcije-test.xlsx").unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet2") {
        for row in r.rows().skip(1).take(240) {
            // println!("row={:?}, row[0]={:?}", row, row[0]);
            questions.push(Question1::new((
                row[0].get_float().unwrap() as i32,
                row[1].get_float().unwrap() as i32,
                row[2].get_float().unwrap() as i32,
                row[3].to_string(),
                row[4].to_string(),
                row[5].to_string(),
            )));
        }
    }

    let questionare = Questionaire::new(questions);
    println!(
        "\nneuroticism = {:?}",
        questionare.get_neuroticism_questions()
    );

    if !SKIP_CHAT_GPT {
        let url: &str = "https://api.openai.com/v1/chat/completions";
        let prompt = Prompt {
        model: String::from("gpt-3.5-turbo"),
        messages: vec![Message {
            role: String::from("system"),
            content: String::from("you are pshychologist assistant and know everything about NEO Personality Inventory Revised (NEO-PI-R)"),
        }, Message {
            role: String::from("user"),
            content: String::from("On the question 'I'm not the one who cares a lot' patient answere with 4. Describe the personality by using the NEO-PI-R!"),
        }],
    };
        let res = get_chat_gptresponse(url, &prompt).await;

        println!("ChatGPT response = {:#?}", res);
    }
}

async fn get_chat_gptresponse(
    url: &str,
    prompt: &Prompt,
) -> Result<GptResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(prompt)
        .header(
            "Authorization",
            "Bearer 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",
        )
        .header("content-type", "application/json")
        .send()
        .await?
        .json::<GptResponse>()
        .await?;

    Ok(response)
}
