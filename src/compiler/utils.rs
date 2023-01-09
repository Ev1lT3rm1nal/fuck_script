use crate::lexer::Position;

pub fn string_with_arrows(text: String, pos_start: &Position, pos_end: &Position) -> String {
    let mut result = String::new();

    // Calculate indices
    // substring from pos_start to pos_end
    let sub = text[..pos_end.index as usize].to_string();
    let mut idx_start = sub.rfind('\n').unwrap_or(0);
    let sub = text[(pos_start.index as usize + 1)..pos_end.index as usize].to_string();
    let mut idx_end = sub.find('\n').unwrap_or(sub.len()) + pos_start.index as usize + 1;

    // Generate each line
    let line_count = pos_end.ln - pos_start.ln + 1;
    for i in 0..line_count {
        // Calculate line columns
        let line = &text[idx_start..idx_end];
        let col_start = if i == 0 { pos_start.col } else { 0 };
        let col_end = if i == line_count - 1 {
            pos_end.col as usize
        } else {
            line.len() - 1
        };

        /*
         # Append to result
        result += line + '\n'
        result += ' ' * col_start + '^' * (col_end - col_start)
         */
        result.push_str(line);
        result.push_str("\n");
        result.push_str(&" ".repeat(col_start as usize));
        result.push_str(&"^".repeat(col_end - col_start as usize));

        /*
        # Re-calculate indices
        idx_start = idx_end
        idx_end = text.find('\n', idx_start + 1)
        if idx_end < 0: idx_end = len(text)
         */
        idx_start = idx_end;
        let text2 = &text[idx_start + 1..];
        idx_end = text2.find('\n').unwrap_or(text2.len());
    }

    return result.replace("\t", "");
}
