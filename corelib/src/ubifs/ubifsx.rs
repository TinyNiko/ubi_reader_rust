use crate::ubi_io::UbiFile;
pub struct UbiFs {
    name: &'a str,
    path: &'a str
}

impl UbiFs {
    pub fn new(ubifs_file: UbiFile)-> UbiFs{
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