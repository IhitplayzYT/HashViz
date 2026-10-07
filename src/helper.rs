pub mod Helper{
    use std::{collections::HashMap, process::exit};

    const DBG_STR: &str = "";
    const OK:i32 = 0;
    const ERR:i32 = -1;


    #[derive(Debug,Clone)]
    pub struct CLI{
        pub dbg: bool,
        pub chr_hist: Vec<(u8,usize)>,
        pub dimx: usize,
        pub dimy: usize,
        pub hash: String
    }


    pub fn Help(){
        println!("{DBG_STR}");
        exit(OK);
    }


    impl CLI{
        pub fn new() -> Self{
            Self {dbg: false,chr_hist:vec![],dimx:40,dimy:16,hash: String::new()}
        }

        pub fn Parse_Args(&mut self){
            let args: Vec<String> = std::env::args().skip(1).collect();
            let mut seen = HashMap::new();
            for i in &args{
                if i == "-d" || i == "--debug" || i == " --DEBUG" || i == "-D"{
                    self.dbg = true;
                } else if i == "-h" || i == "--help" || i == " --HELP" || i == "-H"{
                    Help();
                } else if i.starts_with("--str="){
                    let k = i[i.find("=").unwrap()+1..].bytes().collect::<Vec<u8>>();
                    for i in k{
                        if let Some(idx) = seen.get(&i){
                            self.chr_hist[*idx as usize].1 += 1;
                        }else{
                            self.chr_hist.push((i,1));
                            let l = self.chr_hist.len();
                            seen.insert(i, l-1);
                        }
                    }
                } else if i.starts_with("--dimx=") || i.starts_with("-x="){
                    self.dimx = i[i.find("=").unwrap()+1..].parse().expect("Dim is a unsigned int")
                } else if i.starts_with("--dimy=") || i.starts_with("-y="){
                    self.dimy = i[i.find("=").unwrap()+1..].parse().expect("Dim is a unsigned int")
                } else{
                    self.hash = i.to_string();
                }
            } 
            if self.hash.is_empty(){
                Help();
            }

            if self.chr_hist.is_empty(){
                for i in self.hash.as_bytes(){
                    if let Some(idx) = seen.get(&i){
                        self.chr_hist[*idx as usize].1 += 1;
                    }else{
                        self.chr_hist.push((*i,1));
                        let l = self.chr_hist.len();
                        seen.insert(*i, l-1);
                    }
                }
            }


        }



    }


    





}