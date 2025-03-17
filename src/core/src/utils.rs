use typst::comemo::Track;
use typst::utils::LazyHash;
use typst::World;

pub struct FakeWorld {
    library: LazyHash<typst::Library>,
}

impl FakeWorld {
    pub fn new() -> Self {
        FakeWorld {
            library: LazyHash::new(typst::Library::builder().build()),
        }
    }
}

impl World for FakeWorld {
    fn library(&self) -> &LazyHash<typst::Library> {
        &self.library
    }
    fn book(&self) -> &LazyHash<typst::text::FontBook> {
        unimplemented!();
    }
    fn file(&self, id: typst_syntax::FileId) -> typst::diag::FileResult<typst::foundations::Bytes> {
        unimplemented!();
    }
    fn font(&self, index: usize) -> Option<typst::text::Font> {
        unimplemented!();
    }
    fn main(&self) -> typst_syntax::FileId {
        unimplemented!();
    }
    fn source(&self, id: typst_syntax::FileId) -> typst::diag::FileResult<typst_syntax::Source> {
        unimplemented!();
    }
    fn today(&self, offset: Option<i64>) -> Option<typst::foundations::Datetime> {
        unimplemented!();
    }
}

pub fn eval(world: &dyn typst::World, string: &str) -> Result<typst::foundations::Content, String> {
    let result = typst_eval::eval_string(
        &typst::ROUTINES,
        world.track(),
        string,
        typst::syntax::Span::detached(),
        typst_eval::EvalMode::Math,
        world.library().math.scope().clone(),
    );

    match result {
        Ok(value) => match value {
            typst::foundations::Value::Content(content) => Ok(content),
            _ => Err("Expected content result.".to_string()),
        },
        Err(err) => Err(err[0].message.to_string()),
    }
}

pub fn insert_separator<T: Clone>(list: &[T], separator: T) -> Vec<T> {
    list.iter()
        .flat_map(|x| vec![x.clone(), separator.clone()])
        .take(list.len() * 2 - 1)
        .collect()
}
