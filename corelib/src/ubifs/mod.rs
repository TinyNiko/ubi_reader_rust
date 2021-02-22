pub mod defines;

pub struct UbiFs<'a>{
    name: &'a str,
    path: &'a str
}

impl UbiFs<'_> {
    pub fn new()-> UbiFs<'static>{
        UbiFs{
            name: "233",
            path: "23333"
        }
    }
    fn get_file(&self){

    }
    fn get_superblock(&self){

    }
    fn get_master_node(&self){

    }
    fn get_master_node2(&self){

    }
    fn get_leb_size(&self){

    }
    fn get_min_io_size(&self){

    }

    fn display(&self, tab: &str){

    }
}