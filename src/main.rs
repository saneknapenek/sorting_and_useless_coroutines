use std::io::Error;
use std::fs;

mod sorting;
mod files;


fn main() {
    let list_f_name = vec![String::from("numbers.txt"), String::from("numbers1.txt"), String::from("numbers2.txt"), String::from("numbers3.txt")];
    sort_list_fs(list_f_name);
    clear_dir().unwrap();
}

fn sort_list_fs(fs: Vec<String>) -> Vec<String> {
    if fs.len() == 1 {
        let mut numbers: Vec<u32> = files::readf(&fs.last().unwrap()).unwrap();
        sorting::quicksort(&mut numbers, None);
        files::writef("result.txt", numbers).unwrap();
        return vec!["result".to_string()]
    }
    let mut lfs: Vec<String> = Vec::new();
    for i in (0..(fs.len()-(fs.len()&1))).step_by(2) {
        let mut numbres1 = files::readf(&fs[i]).expect("Error read");
        numbres1.extend(files::readf(&fs[i+1]).expect("Error read"));
        // numbres1.len() + 1
        sorting::quicksort(&mut numbres1, None);
        let nname = concat_f(&fs[i], &fs[i+1]).unwrap();
        lfs.push(nname);
    }
    if fs.len() & 1 == 1 {
        let mut numbers: Vec<u32> = files::readf(&fs.last().unwrap()).unwrap();
        sorting::quicksort(&mut numbers, None);
        let name = format!(".D{}", fs.last().unwrap());
        files::writef(&name, numbers).unwrap();
        lfs.push(name);
    }
    sort_list_fs(lfs)
}

fn concat_f<'a>(file1: &'a str, file2: &'a str) -> Result<String, Error> {
    let mut v1 = files::readf(file1)?;
    let mut v2 = files::readf(file2)?;
    v1.append(&mut v2);
    let name = format!(".D{}&{}", file1, file2);
    files::writef(&name, v1)?;
    Ok(name)
}

fn clear_dir() -> Result<(), Box<dyn std::error::Error>> {
    let files = fs::read_dir("./")?;
    for file in files {
        let file = file?;
        let name = file.file_name();
        let name_str = name.to_str().ok_or("Invalid unicode in file name")?;
        if name_str.starts_with(".D") {
            fs::remove_file(file.path())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::sorting::quicksort;

    #[test]
    fn test_quicksort() {
        let sorted_lv = [334354118, 459781877, 532020425, 687082934, 749535043, 853909619, 856914853, 907183392, 941639991, 947097377, 1045083427, 1068758243, 1130049393, 1131940090, 1194626872, 1503656805, 1982070751, 1998919010, 2038328153, 2093724164];
        let mut lv = [1045083427, 1194626872, 1998919010, 907183392, 687082934, 1068758243, 2038328153, 1130049393, 2093724164, 532020425, 941639991, 334354118, 856914853, 1503656805, 1131940090, 749535043, 853909619, 459781877, 947097377, 1982070751];
        quicksort(&mut lv, None);
        assert_eq!(sorted_lv.first(), lv.first());
        assert_eq!(sorted_lv.last(), lv.last());
    }
}