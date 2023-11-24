mod sorting;
mod files;


fn main() {
    let path = String::from("numbers.txt");
    let mut numbres = files::sreadf(&path).unwrap();
    sorting::quicksort(& mut numbres);
    let res_str: Vec<String> = numbres.into_iter().map(|e| e.to_string()).collect();
    files::swritef(&path, &res_str.join(" ")).unwrap();
}
