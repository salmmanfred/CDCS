pub struct prov{
    pub provinces: Vec<(String, u64)>
}
impl prov{
    pub fn new()->prov{
        prov{
            provinces: Vec::new(),
        }
    }
}