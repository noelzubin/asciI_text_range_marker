const SYMBOLS: [char; 7] = ['╭', '╮', '│', '├', '─', '╭', '┴'];

struct Label {
    label: String,
    from: usize,
    to: usize,
}

impl Label {
    fn new(label: String, from: usize, to: usize) -> Self {
        Label { label, from, to }
    }
}

#[derive(Debug)]
struct LabelInterval {
    level: usize,
    start: usize,
    end: usize,
    label: String,
}

fn main() {
    let buf = std::io::read_to_string(std::io::stdin()).unwrap();

    let (s, mut labels) = parse(&buf);
    let mut first: Vec<char> = " ".repeat(labels.last().unwrap().to + 1).chars().collect();

    for l in labels.iter() {
        if l.to == l.from {
            first[l.from] = SYMBOLS[2];
        } else if l.to == l.from + 1 {
            first[l.from] = SYMBOLS[3];
            first[l.to] = SYMBOLS[1];
        } else {
            first[l.from] = SYMBOLS[0];
            first[l.to] = SYMBOLS[1];
            for i in l.from + 1..l.to {
                first[i] = SYMBOLS[4];
            }
            first[(l.to + l.from) / 2] = SYMBOLS[6];
        }
    }

    let first_str: String = first.into_iter().collect();

    let mut prev_start = 999; // Where the previous label started
    let mut prev_layer = 0; // previous label's layer
    let mut intervals: Vec<LabelInterval> = vec![];

    labels.reverse();
    for label in labels {

        let start = (label.from + label.to) / 2;
        let end = start + label.label.len() + 2;

        if end < prev_start {
            // Can put it here.
            intervals.push(LabelInterval {
                label: label.label,
                level: 0,
                start,
                end,
            });
            prev_layer = 0;
        } else {
            // Add to next layer.
            intervals.push(LabelInterval {
                label: label.label,
                level: prev_layer + 1,
                start,
                end,
            });
            prev_layer = prev_layer + 1;
        }
        prev_start = start;
    }

    let max_level = intervals.iter().map(|i| i.level).max().unwrap();
    let max_length = intervals.iter().map(|i| i.end).max().unwrap();
    let mut label_lines: Vec<Vec<char>> = (0..max_level + 1)
        .map(|_level| " ".repeat(max_length).chars().collect())
        .collect();
    label_lines
        .iter_mut()
        .enumerate()
        .for_each(|(level, line)| {
            for interval in intervals.iter() {
                if interval.level == level {
                    let label_with_symbol = SYMBOLS[0].to_string() + " " + &interval.label;
                    line.splice(
                        interval.start..interval.end,
                        label_with_symbol.chars().collect::<Vec<char>>(),
                    );
                }
            }
        });

    for interval in intervals.iter() {
        for i in 0..interval.level {
            label_lines[i][interval.start] = SYMBOLS[2];
        }
    }

    let label_lines = label_lines
        .into_iter()
        .map(|line| line.iter().collect())
        .rev()
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", label_lines);
    println!("{}", first_str);
    println!("{}", s);
}


fn parse(str: &str) -> (&str, Vec<Label>) {
    let mut lines = str.trim().lines();
    let s = lines.next().unwrap();

    let labels = lines.map(|line| {
        let mut label_parts =  line.split(":");
        let from = label_parts.next().unwrap().parse().unwrap();
        let to = label_parts.next().unwrap().parse().unwrap();
        let label = label_parts.next().unwrap();
        Label::new(label.to_string(), from, to)
    }).collect::<Vec<Label>>();

    (s, labels)
}