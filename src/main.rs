use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug)]
struct TextAnalysis {
    word_count: usize,
    char_count: usize,
    keyword_density: Vec<(String, f64)>,
    spelling_errors: Vec<String>,
    ai_probability: f64,
    readability_score: f64,
    sentiment_score: f64,
    summary: String,
}

struct TextAnalyzer {
    common_words: HashSet<String>,
    common_errors: HashMap<String, String>,
    ai_indicators: Vec<String>,
}

impl TextAnalyzer {
    fn new() -> Self {
        Self {
            common_words: HashSet::from_iter(vec![
                "o", "a", "os", "as", "um", "uma", "uns", "umas",
                "de", "do", "da", "dos", "das", "em", "no", "na",
                "nos", "nas", "por", "para", "com", "sem", "sob",
                "sobre", "entre", "que", "e", "ou", "mas", "se",
                "porque", "como", "quando", "onde", "qual", "quais",
                "este", "esta", "esse", "essa", "aquele", "aquela",
                "isto", "isso", "aquilo"
            ].into_iter().map(|s| s.to_string())),
            
            common_errors: HashMap::from_iter(vec![
                ("concerteza".to_string(), "com certeza".to_string()),
                ("menas".to_string(), "menos".to_string()),
                ("fazem".to_string(), "faz".to_string()),
                ("houveram".to_string(), "houve".to_string()),
                ("imformação".to_string(), "informação".to_string()),
                ("advinhar".to_string(), "adivinhar".to_string()),
                ("envia-lo".to_string(), "enviá-lo".to_string()),
                ("provalvelmente".to_string(), "provavelmente".to_string()),
            ]),
            
            ai_indicators: vec![
                "além disso".to_string(),
                "dessa forma".to_string(),
                "por outro lado".to_string(),
                "é importante destacar".to_string(),
                "vale ressaltar".to_string(),
                "convém mencionar".to_string(),
                "nesse contexto".to_string(),
                "de acordo com".to_string(),
            ],
        }
    }

    fn analyze_text(&self, text: &str) -> TextAnalysis {
        let words = self.extract_words(text);
        let word_count = words.len();
        let char_count = text.chars().count();
        
        let keyword_density = self.analyze_keywords(&words);
        let spelling_errors = self.check_spelling(&words);
        let ai_probability = self.detect_ai_patterns(text);
        let readability_score = self.calculate_readability(text, word_count);
        let sentiment_score = self.analyze_sentiment(text);
        let summary = self.generate_summary(text);
        
        TextAnalysis {
            word_count,
            char_count,
            keyword_density,
            spelling_errors,
            ai_probability,
            readability_score,
            sentiment_score,
            summary,
        }
    }
    
    fn extract_words(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphabetic() || c == ' ' { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }
    
    fn analyze_keywords(&self, words: &[String]) -> Vec<(String, f64)> {
        let mut word_freq = HashMap::new();
        let total_words = words.len() as f64;
        
        for word in words {
            if word.len() > 3 && !self.common_words.contains(word) {
                *word_freq.entry(word.clone()).or_insert(0) += 1;
            }
        }
        
        let mut densities: Vec<(String, f64)> = word_freq
            .into_iter()
            .map(|(word, count)| (word, (count as f64 / total_words) * 100.0))
            .filter(|(_, density)| *density > 1.0)
            .collect();
            
        densities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        densities.truncate(10);
        
        densities
    }
    
    fn check_spelling(&self, words: &[String]) -> Vec<String> {
        let mut errors = Vec::new();
        
        for word in words {
            if let Some(correct) = self.common_errors.get(word) {
                errors.push(format!("'{}' → '{}'", word, correct));
            }
        }
        
        errors
    }
    
    fn detect_ai_patterns(&self, text: &str) -> f64 {
        let text_lower = text.to_lowercase();
        let mut ai_score: f64 = 0.0; // ESPECIFICADO o tipo aqui
        
        // Padrões comuns em texto de IA
        for indicator in &self.ai_indicators {
            if text_lower.contains(indicator) {
                ai_score += 12.5; // Cada indicador adiciona 12.5%
            }
        }
        
        // Verificar repetição excessiva
        let sentences: Vec<&str> = text.split(|c| c == '.' || c == '!' || c == '?').collect();
        if sentences.len() > 2 {
            let avg_length = text.len() as f64 / sentences.len() as f64;
            if avg_length > 120.0 {
                ai_score += 15.0; // Frases muito longas
            }
        }
        
        // Verificar estrutura muito formal
        let formal_words = vec!["portanto", "entretanto", "contudo", "todavia"];
        for word in formal_words {
            if text_lower.contains(word) {
                ai_score += 5.0;
            }
        }
        
        if ai_score > 100.0 {
            100.0
        } else {
            ai_score
        }
    }
    
    fn calculate_readability(&self, text: &str, word_count: usize) -> f64 {
        if word_count == 0 {
            return 0.0;
        }
        
        let sentence_count = text.split(|c| c == '.' || c == '!' || c == '?').count().max(1);
        let avg_sentence_length = word_count as f64 / sentence_count as f64;
        
        let long_words = text.split_whitespace()
            .filter(|word| word.len() > 6)
            .count();
        let long_word_ratio = long_words as f64 / word_count as f64;
        
        let mut score = 100.0;
        
        // Penalizar frases muito longas
        if avg_sentence_length > 25.0 {
            score -= (avg_sentence_length - 25.0) * 2.0;
        }
        
        // Penalizar muitas palavras longas
        if long_word_ratio > 0.2 {
            score -= (long_word_ratio - 0.2) * 100.0;
        }
        
        // Bonificar frases bem equilibradas
        if avg_sentence_length >= 15.0 && avg_sentence_length <= 20.0 {
            score += 10.0;
        }
        
        if score > 100.0 {
            100.0
        } else if score < 0.0 {
            0.0
        } else {
            score
        }
    }
    
    fn analyze_sentiment(&self, text: &str) -> f64 {
        let positive_words: HashSet<&str> = HashSet::from_iter(vec![
            "bom", "boa", "ótimo", "excelente", "maravilhoso", "incrível",
            "gosto", "amo", "adoro", "feliz", "alegre", "contento",
            "sucesso", "vitória", "conquista", "progresso", "evolução",
            "bonito", "lindo", "perfeito", "ideal", "fantástico"
        ]);
        
        let negative_words: HashSet<&str> = HashSet::from_iter(vec![
            "ruim", "péssimo", "horrível", "terrível", "ódio", "detesto",
            "triste", "deprimido", "angustiado", "preocupado", "medo",
            "fracasso", "derrota", "problema", "dificuldade", "erro",
            "feio", "horroroso", "desastre", "catástrofe", "pior"
        ]);
        
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut positive_count = 0;
        let mut negative_count = 0;
        let mut total_scored = 0;
        
        for word in words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
            if positive_words.contains(clean_word.as_str()) {
                positive_count += 1;
                total_scored += 1;
            } else if negative_words.contains(clean_word.as_str()) {
                negative_count += 1;
                total_scored += 1;
            }
        }
        
        if total_scored == 0 {
            return 50.0; // Neutro
        }
        
        let sentiment_ratio = positive_count as f64 / total_scored as f64;
        let score = sentiment_ratio * 100.0;
        
        if score > 100.0 {
            100.0
        } else if score < 0.0 {
            0.0
        } else {
            score
        }
    }
    
    fn generate_summary(&self, text: &str) -> String {
        let sentences: Vec<&str> = text.split(|c| c == '.' || c == '!' || c == '?').collect();
        
        if sentences.is_empty() {
            return "Texto muito curto para resumo.".to_string();
        }
        
        // Pegar a primeira frase significativa
        let first_sentence = sentences[0].trim();
        let words: Vec<&str> = first_sentence.split_whitespace().collect();
        
        if words.len() <= 5 {
            // Se a primeira frase for muito curta, tentar a segunda
            if sentences.len() > 1 {
                return self.truncate_to_chars(sentences[1].trim(), 100);
            }
        }
        
        self.truncate_to_chars(first_sentence, 100)
    }
    
    fn truncate_to_chars(&self, text: &str, max_chars: usize) -> String {
        if text.chars().count() <= max_chars {
            return text.to_string();
        }
        
        let mut result = String::new();
        let mut char_count = 0;
        
        for c in text.chars() {
            if char_count + 1 > max_chars {
                break;
            }
            result.push(c);
            char_count += 1;
        }
        
        result.trim().to_string() + "..."
    }
}

fn main() {
    println!("🔍 ANALISADOR DE TEXTO AVANÇADO");
    println!("===============================\n");
    
    let analyzer = TextAnalyzer::new();
    
    loop {
        println!("Digite o texto para análise (máx 2000 caracteres):");
        println!("(ou 'sair' para encerrar)");
        println!("{}", "─".repeat(50));
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("sair") {
            println!("👋 Encerrando analisador...");
            break;
        }
        
        if input.is_empty() {
            println!("❌ Texto vazio. Tente novamente.\n");
            continue;
        }
        
        if input.chars().count() > 2000 {
            println!("⚠️  Texto muito longo. Analisando apenas os primeiros 2000 caracteres.\n");
        }
        
        let text = if input.chars().count() > 2000 {
            input.chars().take(2000).collect::<String>()
        } else {
            input.to_string()
        };
        
        println!("\n🔄 Analisando texto...");
        let analysis = analyzer.analyze_text(&text);
        
        display_results(&analysis);
        
        println!("\n{}", "═".repeat(60));
        println!();
    }
}

fn display_results(analysis: &TextAnalysis) {
    println!("\n📊 RESULTADOS DA ANÁLISE");
    println!("{}", "─".repeat(30));
    
    println!("📝 Estatísticas Básicas:");
    println!("   • Palavras: {}", analysis.word_count);
    println!("   • Caracteres: {}", analysis.char_count);
    
    println!("\n🎯 Palavras-chave Principais:");
    if analysis.keyword_density.is_empty() {
        println!("   Nenhuma palavra-chave significativa encontrada");
    } else {
        for (i, (keyword, density)) in analysis.keyword_density.iter().enumerate().take(5) {
            println!("   {}. {} ({:.1}%)", i + 1, keyword, density);
        }
    }
    
    println!("\n🔍 Qualidade do Texto:");
    println!("   • Legibilidade: {:.1}%", analysis.readability_score);
    match analysis.readability_score {
        0.0..=49.9 => println!("     ⚠️  Difícil de entender"),
        50.0..=69.9 => println!("     ✅ Nível moderado"),
        70.0..=84.9 => println!("     ✅ Bom entendimento"),
        _ => println!("     🎉 Excelente clareza"),
    }
    
    println!("\n😊 Análise de Sentimento:");
    println!("   • Score: {:.1}%", analysis.sentiment_score);
    match analysis.sentiment_score {
        0.0..=29.9 => println!("     😞 Negativo"),
        30.0..=44.9 => println!("     😐 Levemente negativo"),
        45.0..=55.0 => println!("     😐 Neutro"),
        55.1..=70.0 => println!("     🙂 Levemente positivo"),
        _ => println!("     😊 Positivo"),
    }
    
    println!("\n🤖 Detecção de IA:");
    println!("   • Probabilidade: {:.1}%", analysis.ai_probability);
    match analysis.ai_probability {
        0.0..=30.0 => println!("     👤 Provavelmente humano"),
        30.1..=60.0 => println!("     🤔 Inconclusivo"),
        60.1..=80.0 => println!("     ⚠️  Possível IA"),
        _ => println!("     🤖 Provavelmente IA"),
    }
    
    println!("\n✏️  Erros Ortográficos:");
    if analysis.spelling_errors.is_empty() {
        println!("   ✅ Nenhum erro comum detectado");
    } else {
        for error in &analysis.spelling_errors {
            println!("   • {}", error);
        }
    }
    
    println!("\n📋 Resumo:");
    println!("   \"{}\"", analysis.summary);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = TextAnalyzer::new();
        assert!(!analyzer.common_words.is_empty());
    }

    #[test]
    fn test_text_analysis() {
        let analyzer = TextAnalyzer::new();
        let text = "Rust é uma linguagem de programação excelente para sistemas seguros.";
        let analysis = analyzer.analyze_text(text);
        
        assert!(analysis.word_count > 0);
        assert!(analysis.readability_score > 0.0);
    }

    #[test]
    fn test_sentiment_analysis() {
        let analyzer = TextAnalyzer::new();
        
        let positive_text = "Isso é bom e excelente!";
        let positive_analysis = analyzer.analyze_text(positive_text);
        assert!(positive_analysis.sentiment_score > 60.0);
        
        let negative_text = "Isso é ruim e terrível!";
        let negative_analysis = analyzer.analyze_text(negative_text);
        assert!(negative_analysis.sentiment_score < 40.0);
    }
}