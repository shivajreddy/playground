

fn main() {
    
    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    
    let mut p_map: [usize; 26] = [0; 26];
    p.chars().for_each(|item| p_map[item as usize - 97] += 1);
    
    let mut result: Vec<i32> = Vec::new();
    
    let s_letters: Vec<char> = Vec::new();
    
    let (mut l, mut r) = (0, 0);
    let mut win_map: [usize; 26] = [0; 26];
    
    while r < s.len() {
        let right_idx = s_letters[r] as usize - 97;
        win_map[right_idx] += 1;
        
        if r - l + 1 == p.len() {
            
            if same_list(&win_map, &p_map) {
                result.push(l as i32);
            }
            
            let left_idx = s_letters[l] as usize - 97;
            win_map[left_idx] -= 1;
            l += 1;
        }
        
        r += 1;
    }

}

fn same_list(a: &[usize; 26], b: &[usize; 26]) -> bool {
    for idx in (0..27) {
        if a[idx] != b[idx] {
            return false;
        }
    }
    true
}
