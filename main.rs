use csv::Reader;
mod graph;

fn parse_csv(path: &str, graph: &mut graph::Graph) -> anyhow::Result<()> {
    let file = std::fs::File::open(path)?;
    let mut rdr = Reader::from_reader(file);
    for row in rdr.records().skip(1) {
        let t1: &[_] = &['[', ']'];
        let _ = row.map(|r| {
            if r.len() == 13 {
                let hashtags = &r[10];
                //println!("==: {}", hashtags);
                let splits =
                    hashtags.trim_matches(t1).split(',').collect::<Vec<_>>();
                for (i, v) in splits.iter().enumerate() {
                    for v2 in splits.iter().skip(i) {
                        let label1 = v
                            .trim()
                            .trim_start_matches('\'')
                            .trim_end_matches('\'')
                            .to_ascii_lowercase(); //cant use trim_matches, beacuse ' may be in mid character
                        let label2 = v2
                            .trim()
                            .trim_start_matches('\'')
                            .trim_end_matches('\'')
                            .to_ascii_lowercase();

                        graph.add_edge(&label1, &label2);
                    }
                }
            }
        });
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut g = graph::Graph::new();
    //for file in ["Bitcoin_tweets_dataset_2.csv", "Bitcoin_tweets.csv"] {
    for file in ["Bitcoin_tweets_dataset_2.csv"] {
        parse_csv(file, &mut g)?;
    }

    //println!("{}", g);
    let _alls = g.dijkstra("btc");
    println!("{:?}", _alls["bitcoin"]);
    let path = g.path("btc", "bitcoin");
    println!("{:?}", path);
    Ok(())
}
