use crate::staminars::tokens::position::Position;
use std::cmp::max;

/// This function is used to add up arrows under the error
pub(crate) fn string_with_arrows(
        text:String,
        pos_start:Position,
        pos_end:Position
    ) -> String {
    let mut result = String::new();
    
    //Calculate indices
    let pos_start_string = rfind(&text, '\n', pos_start.idx, pos_end.idx);
    let mut pos_end_string = rfind(&text, '\n', pos_start.idx + 1, text.len() as u32);
    let mut idx_start:u32 = 0;
    let mut idx_end:u32 = 0;

    if pos_start_string == None {
        idx_start = 0;
    } else {
        idx_start = max(pos_start_string.unwrap(), 0) as u32;
    }
    if pos_end_string == None {
        idx_end = text.len() as u32;
    } else {
        idx_end = pos_end_string.unwrap() as u32;
        if idx_end == idx_start {
            idx_end = text.len() as u32;
        }
    }

    //Generate each line
    let line_count: u32 = pos_end.ln - pos_start.ln + 1;
    for i in 0..line_count {
        //Calculate line columns
        let line = text[idx_start as usize..idx_end as usize].to_string();
        let col_start = if i == 0 {
            pos_start.col
        } else {
            0
        };
        let col_end = if i == line_count - 1 {
            pos_end.col
        } else {
            line.len() as u32 - 1
        };
        //Append to result
        result.push_str(&format!("{}\n", line));
        //Create the blank space with col_start
        let mut blank_space:String = String::new();
        for _ in 0..col_start {
            blank_space.push(' ');
        }
        let mut up_arrow:String = String::new();
        for _ in 0..col_end - col_start {
            up_arrow.push('^');
        }
        result.push_str(&format!("{}{}", blank_space, up_arrow));
        //Recalculate indices
        idx_start = idx_end;
        pos_end_string = rfind(&text, '\n', idx_start + 1, text.len() as u32);
        if pos_end_string == None {
            idx_end = text.len() as u32;
        } else {
            idx_end = pos_end_string.unwrap() as u32;
            if idx_end == idx_start {
                idx_end = text.len() as u32;
            }
        }
    }

    return result.replace("\t", "")

}

/// This function is used to find the first occurence of a character in a string between two indices
fn rfind(s: &str, chr: char, pos_start:u32, pos_end:u32) -> Option<usize> {
    let st = s.to_string();
    if st.len() < pos_start as usize {
        return None
    }
    let mut vec:Vec<&str> = st.split("").collect();
    let mut cpt: usize = 0;
    for i in 0..vec.len() {
        if i < pos_start as usize {
            vec.remove(i - 1);
            cpt += 1;
        }
    }
    if vec.len() > pos_end as usize {
        for i in pos_end as usize - pos_start as usize..vec.len() -1 {
            vec.remove(i - 1);
        }
    }
    for i in 0..vec.len() {
        if vec[i] == chr.to_string() {
            println!("{} {}", i, cpt);
            return Some(i + cpt)
        }
    }
    return None
}