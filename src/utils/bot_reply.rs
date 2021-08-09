use rand::seq::SliceRandom;

pub fn reply_question(question: String) -> String {
    let answers: Vec<String> = vec![
        // Normal answers
        "As I see it, yes.".into(),
        "Ask again later.".into(),
        "Better not tell you now.".into(),
        "Cannot predict now.".into(),
        "Concentrate and ask again.".into(),
        "Don’t count on it.".into(),
        "It is certain.".into(),
        "It is decidedly so.".into(),
        "Most likely.".into(),
        "My reply is no.".into(),
        "My sources say no.".into(),
        "Outlook not so good.".into(),
        "Outlook good.".into(),
        "Reply hazy, try again.".into(),
        "Signs point to yes.".into(),
        "Very doubtful.".into(),
        "Without a doubt.".into(),
        "Yes.".into(),
        "Yes – definitely.".into(),
        "You may rely on it.".into(),
        // Gifs
        "https://tenor.com/Keve.gif".into(), // Mind blown
        "https://tenor.com/xnba.gif".into(), // BOOM
        "https://tenor.com/InWt.gif".into(), // Whatever
    ];
    
    answers
        .choose(&mut rand::thread_rng())
        .expect("Problem trying to pick a random vector entry (1)")
        .clone()
}