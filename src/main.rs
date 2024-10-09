use calamine::{open_workbook, Reader, Xlsx};
use sajko::{
    factors::{Question1, Questionaire},
    gpt_models::{GptResponse, Message, Prompt},
};

const SKIP_CHAT_GPT: bool = false;

#[tokio::main]
async fn main() {
    println!("Hello, psycho!");
    let mut questions = vec![];

    // read questionaire Excel file with columns: ordinal number, score, answer, difference, code, question. The resulting array should be of type Vec<(i32, i32, i32, String, String, String)> (example: [1, 1, 1, "S", "n1", "Nisam od onih koji stalno brinu."] )
    let mut excel: Xlsx<_> = open_workbook("questionnaire/neo-pi-r-korekcije-test.xlsx")
        .expect("Can't open neo-pi-r-korekcije-test.xlsx");

    if let Some(Ok(r)) = excel.worksheet_range("Sheet2") {
        const STARTING_ROW: usize = 1;
        let mut current_row = STARTING_ROW;
        for row in r.rows().skip(STARTING_ROW).take(240) {
            questions.push(Question1::new((
                row[0].get_float().expect(&format!(
                    "Can't read 'redni broj' from {}. row",
                    current_row
                )) as i32, // redni broj
                row[1].get_float().expect(&format!(
                    "Can't read 'skor na testu' from {}. row",
                    current_row
                )) as i32, // skor na testu
                row[2]
                    .get_float()
                    .expect(&format!("Can't read 'odgovor' from {}. row", current_row))
                    as i32, // odgovor
                row[3].to_string(), // razlika
                row[4].to_string(), // sifra
                row[5].to_string(), // pitanje
            )));
        }
    }

    let questionare = Questionaire::new(questions);
    println!(
        "\nneuroticism = {:?}",
        questionare.get_factor_questions('n')
    );

    if !SKIP_CHAT_GPT {
        let url: &str = "https://api.openai.com/v1/chat/completions";
        let prompt = Prompt {
            model: String::from("gpt-3.5-turbo"),
            messages: vec![Message {
                role: String::from("system"),
                content: String::from("you are pshychologist assistant and know everything about NEO Personality Inventory Revised (NEO-PI-R)"),
            }, Message {
                role: String::from("system"),
                content: String::from("you will get a patient's answers for the NEO-PI-R questionary and should analize the patient"),
            }, Message {
                role: String::from("system"),
                content: String::from("the questions come in the following format: 'Question1 { ordinal_number: <question ordinal number>, score: <score based on the answer. it can be from 0-4>, answer: <answer>, difference: <difference>, code: <code>, question: <actual question> }'"),
            }, Message {
                role: String::from("system"),
                content: String::from("the questionary might not be complete, so do your best with the answers you get. Also the questiona are in serbian language"),
            }, Message {
                role: String::from("system"),
                content: format!("Neuroticism answers: {:?}", questionare.get_factor_questions('n')),
            }, Message {
                role: String::from("system"),
                content: format!("Extraversion answers: {:?}", questionare.get_factor_questions('e')),
            }, Message {
                role: String::from("system"),
                content: format!("Agreeableness answers: {:?}", questionare.get_factor_questions('a')),
            },Message {
                role: String::from("system"),
                content: format!("Conscientiousness answers: {:?}", questionare.get_factor_questions('c')),
            },Message {
                role: String::from("system"),
                content: format!("Openness answers: {:?}", questionare.get_factor_questions('o')),
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
    let resp = client
        .post(url)
        .json(prompt)
        .header(
            "Authorization",
            "Bearer <token>",
        )
        .header("content-type", "application/json")
        .send()
        .await?
        // .text()
        .json::<GptResponse>()
        .await?;


    // let response = resp.json::<GptResponse>().await?;

    Ok(resp)
}
