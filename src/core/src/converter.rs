use log::warn;
use typst::{foundations::{Chainable, Content, Element, Smart, StyleChain}, math::{EquationElem, MathVariant}};

use crate::{katex, node::*, content::*, utils::insert_separator, symbol};

pub fn convert(root: &Content) -> Node {
    let styles = typst::foundations::StyleChain::default();
    let mut converter = ContentConverter {
        parent: None,
        position: None
    };
    root.accept(&mut converter, &styles)
}

#[derive(Clone)]
pub struct ContentConverter {
    pub parent: Option<Content>,
    pub position: Option<usize>,
}

impl ContentVisitor for ContentConverter {
    fn visit_equation(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_equation();
        Node::Array(elem.body.accept(self, style_chain).into_array())
    }

    fn visit_symbol(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_symbol();
        let name = elem.text;let variant = style_chain.get(Element::of::<EquationElem>(), 6, None, || {
            MathVariant::Serif
        });
        if style_chain.entries().count() == 0 {
            match name {
                '≔' => symbol::define(),
                '≠' => symbol::neq(),
                _ => Node::Node(katex::Symbol::get(katex::Mode::Math, name).create_node())
            }
        } else {
            let bold = style_chain.get(Element::of::<EquationElem>(), 8, None, || false);
            let italic = style_chain.get(Element::of::<EquationElem>(), 9, None, || {
                Smart::<bool>::Custom(true)
            }) == Smart::Custom(true);

            let font = match variant {
                MathVariant::Serif => {
                    if bold && italic {
                        "boldsymbol"
                    } else if bold {
                        "mathbf"
                    } else if italic {
                        "mathit"
                    } else {
                        "mathrm"
                    }
                }
                MathVariant::Sans => {
                    if italic {
                        "mathsfit"
                    } else {
                        "mathsf"
                    }
                },
                MathVariant::Cal => "mathcal",
                MathVariant::Frak => "mathfrak",
                MathVariant::Mono => "mathtt",
                MathVariant::Bb => "mathbb",
            }.to_string();

            let body = match name {
                '≔' => symbol::define(),
                '≠' => symbol::neq(),
                _ => Node::Node(katex::Symbol::get(katex::Mode::Math, name).create_node())
            }.into_node().unwrap();
            
            let node = katex::FontBuilder::default()
                .body(Box::new(body))
                .font(font)
                .build().unwrap().into_node();
            Node::Node(node)
        }
    }

    fn visit_styled(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_styled();
        let new_chain = elem.styles.chain(style_chain);
        elem.child.accept(self, &new_chain)
    }

    fn visit_op(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_op();

        let _text = &elem.text;
        let _limits = elem.limits(*style_chain);

        let name = format!("\\{}", _text.plain_text()).to_string();

        let node = katex::OpBuilder::default()
            .limits(_limits)
            .parent_is_sup_sub(false)
            .symbol(false)
            .name(Some(name))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_mat(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_mat();
        let mut constructor = katex::ArrayConstructor::default();

        for row in &elem.rows {
            constructor.next_row();
            for content in row {
                let node = content.accept(self, style_chain);
                let ordgroup = katex::OrdGroupBuilder::default()
                    .body(node.into_array())
                    .build().unwrap().into_node();
                let styling = katex::StylingBuilder::default()
                    .body([ordgroup].to_vec())
                    .style(katex::StyleStr::Text)
                    .build().unwrap().into_node();
                constructor.push_node(styling);
            }
        }
        let array = constructor.builder().build().unwrap().into_node();
        let delim = elem.delim(*style_chain);
        let leftright = katex::LeftRightBuilder::default()
            .body([array].to_vec())
            .left(delim.open().unwrap().to_string())
            .right(delim.close().unwrap().to_string())
            .build().unwrap().into_node();
        Node::Node(leftright)
    }

    fn visit_vec(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let mut converter = VecConverter::new(content.to_vec());
        converter.convert(self, style_chain)
    }

    fn visit_frac(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_frac();

        let _num = &elem.num;
        let _denom = &elem.denom;

        let numer = katex::OrdGroupBuilder::default()
            .body(_num.accept(self, style_chain).into_array())
            .build().unwrap().into_node();

        let denom = katex::OrdGroupBuilder::default()
            .body(_denom.accept(self, style_chain).into_array())
            .build().unwrap().into_node();

        let node = katex::GenFracBuilder::default()
            .numer(Box::new(numer))
            .denom(Box::new(denom))
            .mode(katex::Mode::Math)
            .has_bar_line(true)
            .size(katex::GenFracSizeType::Auto)
            .continued(false)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_align_point(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let node = katex::OrdGroupBuilder::default().build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_linebreak(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let node = katex::CrBuilder::default()
            .new_line(true)
            .build().unwrap().into_node();
        Node::Array(vec![node])
    }

    fn visit_sequence(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let mut converter = SequenceConverter::new(content);
        converter.convert(self, style_chain)
    }

    fn visit_space(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        Node::Node(katex::SpacingBuilder::default()
            .mode(katex::Mode::Math)
            .text("\\ ".to_string())
            .build().unwrap().into_node())
    }

    fn visit_text(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let mut text_converter = TextConverter::new(content.to_text());
        text_converter.convert()
    }

    fn visit_lr(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_lr();

        let _body = &elem.body;
        let _size = elem.size(*style_chain); // unsupported

        let mut body = _body.accept(self, style_chain).into_array();

        // TODO: Another case to treat is when there's a styling node (\textstyle) as parent,
        // and in this case maybe a context object will be needed.
        let mut is_lr = false;
        for n in body.clone() {
            match n {
                katex::Node::Atom(_) |
                katex::Node::MathOrd(_) |
                katex::Node::TextOrd(_) |
                katex::Node::Font(_) => (),
                _ => { is_lr = true; },
            }
        }

        if !is_lr {
            return Node::Array(body.clone());
        }

        let left = match body.remove(0) {
            katex::Node::Atom(atom) => atom.text,
            _ => panic!("Not an atom!"),
        };
        let right = match body.pop().unwrap() {
            katex::Node::Atom(atom) => atom.text,
            _ => panic!("Not an atom!"),
        };

        let node = katex::LeftRightBuilder::default()
            .body(body)
            .left(left)
            .right(right)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_attach(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_attach();

        let _base = &elem.base;
        let _t = elem.t(*style_chain);
        let _b = elem.b(*style_chain);
        let _tl = elem.tl(*style_chain); // unsupported
        let _bl = elem.bl(*style_chain); // unsupported
        let _tr = elem.tr(*style_chain); // unsupported
        let _br = elem.br(*style_chain); // unsupported
        if _tl.is_some() { warn!("Top left element is unsupported."); }
        if _tr.is_some() { warn!("Top right element is unsupported."); }
        if _bl.is_some() { warn!("Bottom left element is unsupported."); }
        if _br.is_some() { warn!("Bottom right element is unsupported."); }

        let base = _base.accept(self, style_chain).into_node_fallback_ordgroup(katex::Mode::Math);
        let sup = _t.map(|c| c.accept(self, style_chain)).map(|n| n.into_node_fallback_ordgroup(katex::Mode::Math));
        let sub = _b.map(|c| c.accept(self, style_chain)).map(|n| n.into_node_fallback_ordgroup(katex::Mode::Math));

        let node = katex::SupSubBuilder::default()
            .base(Some(Box::new(base)))
            .sup(sup.map(Box::new))
            .sub(sub.map(Box::new))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_binom(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_binom();

        let _upper = &elem.upper;
        let _lower = &elem.lower;

        let numer = katex::OrdGroupBuilder::default()
            .body(_upper.accept(self, style_chain).into_array())
            .build().unwrap().into_node();

        let separator = katex::Symbol::get(katex::Mode::Math, ',').create_node();
        let denom_body_parts: Vec<katex::NodeArray> = elem.lower.iter().map(|c| c.accept(self, style_chain).into_array()).collect();
        let denom_body = insert_separator(&denom_body_parts, [separator].to_vec()).iter().flatten().cloned().collect();
        let denom = katex::OrdGroupBuilder::default()
            .body(denom_body)
            .build().unwrap().into_node();

        let node = katex::GenFracBuilder::default()
            .continued(false)
            .numer(numer)
            .denom(denom)
            .has_bar_line(false)
            .left_delim("(".to_string())
            .right_delim(")".to_string())
            .size(katex::GenFracSizeType::Auto)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_cancel(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_cancel();

        let _body = &elem.body;
        let _length = elem.length(*style_chain); // unsupported
        let _inverted = elem.inverted(*style_chain); // unsupported
        let _cross = elem.cross(*style_chain); // unsupported
        let _angle = elem.angle(*style_chain); // unsupported
        let _stroke = elem.stroke(*style_chain); // unsupported

        let body = katex::OrdGroupBuilder::default()
            .body(_body.accept(self, style_chain).into_array())
            .build().unwrap().into_node();

        let node = katex::EncloseBuilder::default()
            .label("\\cancel".to_string())
            .body(Box::new(body))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_cases(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let mut converter = CasesConverter::new(content.to_cases());
        converter.convert(self, style_chain)
    }

    fn visit_limits(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_limits();

        let _body = &elem.body;
        let _inline = elem.inline(*style_chain); // unsupported

        let body = _body.accept(self, style_chain).into_array();

        // This comes inside an AttachElem, so we have to transform this into an operator
        let node = katex::OpBuilder::default()
            .mode(katex::Mode::Math)
            .limits(true)
            .parent_is_sup_sub(false)
            .symbol(false)
            .body(Some(body))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_scripts(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_scripts();

        let _body = &elem.body;

        let body = _body.accept(self, style_chain).into_array();

        let node = katex::OpBuilder::default()
            .mode(katex::Mode::Math)
            .limits(false)
            .parent_is_sup_sub(false)
            .symbol(false)
            .body(Some(body))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_mid(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_mid();

        let _body = &elem.body;

        let delim = _body.plain_text().to_string();
        let node = katex::MiddleBuilder::default()
            .delim(delim)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_overbrace(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_overbrace();

        let _body = &elem.body;
        let _annotation = elem.annotation(*style_chain);

        let base = katex::HorizBraceBuilder::default()
            .label("\\overbrace".to_string())
            .is_over(true)
            .base(Box::new(_body.accept(self, style_chain).into_ordgroup(katex::Mode::Math).into_node()))
            .build().unwrap().into_node();
        let sup = _annotation.map(|c| c.accept(self, style_chain).into_ordgroup(katex::Mode::Math).into_node());

        let node = katex::SupSubBuilder::default()
            .base(Box::new(base))
            .sup(sup.map(Box::new))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_overline(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_overline();

        let _body = &elem.body;

        let body = _body.accept(self, style_chain).into_ordgroup(katex::Mode::Math).into_node();
        let node = katex::OverlineBuilder::default()
            .body(Box::new(body))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_root(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_root();

        let _index = elem.index(*style_chain);
        let _radicand = &elem.radicand;

        let index = _index.map(|c| c.accept(self, style_chain).into_ordgroup(katex::Mode::Math).into_node());
        let body = _radicand.accept(self, style_chain).into_ordgroup(katex::Mode::Math).into_node();

        let node = katex::SqrtBuilder::default()
            .body(Box::new(body))
            .index(index.map(Box::new))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_underbrace(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_underbrace();

        let _body = &elem.body;
        let _annotation = elem.annotation(*style_chain);

        let base = katex::HorizBraceBuilder::default()
            .base(Box::new(_body.accept(self, style_chain).into_ordgroup(katex::Mode::Math).into_node()))
            .is_over(false)
            .label("\\underbrace".to_string())
            .build().unwrap().into_node();
        let sub = _annotation.map(|c| c.accept(self, style_chain).into_ordgroup(katex::Mode::Math).into_node());

        let node = katex::SupSubBuilder::default()
            .base(Box::new(base))
            .sub(sub.map(Box::new))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_underline(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_underline();

        let _body = &elem.body;

        let body = _body.accept(self, style_chain).into_ordgroup(katex::Mode::Math).into_node();

        let node = katex::UnderlineBuilder::default()
            .body(Box::new(body))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_h(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_h();

        let _amount = &elem.amount;
        let _weak = elem.weak(*style_chain); // unsupported

        let length = match _amount {
            typst::layout::Spacing::Fr(fr) => unimplemented!(),
            typst::layout::Spacing::Rel(rel) => rel.abs,
        };

        let node = katex::KernBuilder::default()
            .dimension(katex::Measurement {
                number: length.em.get() as f32,
                unit: "em".to_string(),
            })
            .mode(katex::Mode::Math)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_underbracket(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        // unsupported
        unimplemented!()
    }

    fn visit_overbracket(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        // unsupported
        unimplemented!()
    }

    fn visit_class(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        // unsupported
        unimplemented!()
    }

    fn visit_primes(&mut self, content: &Content, style_chain: &StyleChain) -> Node {
        let elem = content.to_primes();

        let node = katex::OrdGroupBuilder::default()
            .body(vec![katex::Symbol::get(katex::Mode::Math, '′').create_node(); elem.count])
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_accent(&mut self, content: &Content, style_chain: &StyleChain) -> Node { 
        let elem = content.to_accent();

        let _label = match elem.accent.0 {
            '\u{300}' => "\\grave",
            '\u{301}' => "\\acute",
            '\u{302}' => "\\hat",
            '\u{303}' => "\\tilde",
            '\u{304}' => "\\bar",
            '\u{305}' => "\\overline",
            '\u{306}' => "\\breve",
            '\u{307}' => "\\dot",
            '\u{308}' => "\\ddot",
            '\u{20db}' => "\\dddot",
            '\u{20dc}' => "\\ddddot",
            '\u{30a}' => "\\mathring",
            '\u{30b}' => "\\H",
            '\u{30c}' => "\\check",
            '\u{20d7}' => "\\overrightarrow",
            '\u{20d6}' => "\\overleftarrow",
            '\u{20e1}' => "\\overleftrightarrow",
            '\u{20d1}' => "\\overrightharpoon",
            '\u{20d0}' => "\\overleftharpoon",
            _ => unimplemented!(),
        }.to_string();

        let _base = &elem.base;
        let node = katex::AccentBuilder::default()
            .label(_label)
            .is_stretchy(Some(true))
            .is_shifty(Some(false))
            .base(Box::new(_base.accept(self, style_chain).into_ordgroup(katex::Mode::Math).into_node()))
            .build().unwrap().into_node();
        Node::Node(node)
    }
}

pub struct SequenceConverter<'a> {
    pub content: &'a Content,
    pub body: Vec<Vec<Node>>,
    pub stack: Vec<Node>,
    pub is_aligned: bool,
}

impl<'a> SequenceConverter<'a> {
    pub fn new(content: &'a Content) -> Self {
        Self {
            content,
            body: Vec::new(),
            stack: Vec::new(),
            is_aligned: false,
        }
    }

    pub fn convert(&mut self, visitor: &mut ContentConverter, style_chain: &StyleChain) -> Node {
        self.process_sequence_elements(visitor, style_chain);

        if self.is_aligned {
            self.convert_align()
        } else {
            self.convert_flatten()
        }
    }

    pub fn convert_flatten(&mut self) -> Node {
        let nodes = self.body.iter().flatten().flat_map(|n| n.clone().into_array());
        Node::Array(nodes.collect())
    }

    pub fn convert_align(&mut self) -> Node {
        let mut constructor = katex::ArrayConstructor::default();

        for row in self.body.iter_mut() {
            constructor.next_row();
            for node in row.iter_mut() {
                let ordgroup = katex::OrdGroupBuilder::default()
                    .body(node.clone().into_array())
                    .build().unwrap().into_node();
                let styling = katex::StylingBuilder::default()
                    .style(katex::StyleStr::Display)
                    .body([ordgroup].to_vec())
                    .build().unwrap().into_node();
                constructor.push_node(styling)
            }
        }
        constructor.cols_leftright_align();

        let array = constructor.builder()
            .add_jot(true)
            .leqno(false)
            .col_separation_type(katex::ColSeparationType::Align)
            .build().unwrap().into_node();
        Node::Node(array)
    }

    pub fn process_sequence_elements(&mut self, visitor: &mut ContentConverter, style_chain: &StyleChain) {
        let sequence = self.content.to_sequence().unwrap();

        for (i, elem) in sequence.enumerate() {
            if elem.is_linebreak() || elem.is_align_point() {
                self.dump_stack_onto_body();
                if elem.is_linebreak() {
                    self.body.push(Vec::new());
                }
                if elem.is_align_point() {
                    self.is_aligned = true;
                }
            }
            visitor.parent = Some(self.content.clone());
            visitor.position = Some(i);
            let node = elem.accept(visitor, style_chain);
            self.stack.push(node);
        }
        self.dump_stack_onto_body();
    }

    pub fn dump_stack_onto_body(&mut self) {
        if self.body.is_empty() {
            self.body.push(Vec::new())
        }
        let nodes = self.stack.iter().flat_map(|n| n.clone().into_array()).collect();
        self.body.last_mut().unwrap().push(Node::Array(nodes));
        self.stack.clear();
    }
}

pub struct CasesConverter<'a> {
    pub elem: &'a typst::math::CasesElem,
    pub body: Vec<Vec<Node>>,
    pub stack: Vec<Node>,
}

impl<'a> CasesConverter<'a> {
    pub fn new(elem: &'a typst::math::CasesElem) -> Self {
        Self {
            elem,
            body: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn convert(&mut self, visitor: &mut ContentConverter, style_chain: &StyleChain) -> Node {
        self.process_children(visitor, style_chain);

        let mut constructor = katex::ArrayConstructor::default();
        for row in self.body.iter_mut() {
            constructor.next_row();
            for node in row {
                let ordgroup = katex::OrdGroupBuilder::default()
                    .body(node.clone().into_array())
                    .build().unwrap().into_node();
                let styling = katex::StylingBuilder::default()
                    .style(katex::StyleStr::Text)
                    .body([ordgroup].to_vec())
                    .build().unwrap().into_node();
                constructor.push_node(styling);
            }
        }

        let cols = vec![
            katex::AlignSpec::Align(katex::Align {
                align: "l".to_string(),
                pregap: Some(0f32),
                postgap: Some(1f32),
            }),
            katex::AlignSpec::Align(katex::Align {
                align: "l".to_string(),
                pregap: Some(0f32),
                postgap: Some(0f32),
            }),
        ];
        let array = constructor.builder()
            .arraystretch(1.2)
            .cols(cols)
            .build().unwrap().into_node();
        let leftright = katex::LeftRightBuilder::default()
            .body([array].to_vec())
            .left("\\{".to_string())
            .right(".".to_string())
            .build().unwrap().into_node();

        Node::Node(leftright)
    }

    pub fn process_children(&mut self, visitor: &mut ContentConverter, style_chain: &StyleChain) {
        for child in &self.elem.children {
            if child.is_sequence() {
                let mut converter = SequenceConverter::new(child);
                converter.process_sequence_elements(visitor, style_chain);
                self.body.extend(converter.body);
            } else {
                self.body.push([child.accept(visitor, style_chain)].to_vec());
            }
        }
    }
}

pub struct VecConverter<'a> {
    pub elem: &'a typst::math::VecElem,
}

impl<'a> VecConverter<'a> {
    pub fn new(elem: &'a typst::math::VecElem) -> Self {
        Self {
            elem,
        }
    }

    pub fn convert(&mut self, visitor: &mut ContentConverter, style_chain: &StyleChain) -> Node {
        let mut constructor = katex::ArrayConstructor::default();

        for content in &self.elem.children {
            constructor.next_row();
            let node = content.accept(visitor, style_chain).into_node().unwrap();
            let ordgroup = katex::OrdGroupBuilder::default()
                .body([node].to_vec())
                .build().unwrap().into_node();
            let styling = katex::StylingBuilder::default()
                .body([ordgroup].to_vec())
                .style(katex::StyleStr::Text)
                .build().unwrap().into_node();
            constructor.push_node(styling);
        }
        constructor.cols_center_align();

        let mut builder = constructor.builder();
        let array = builder
            .hskip_before_and_after(false)
            .row_gaps([None].to_vec())
            .build().unwrap().into_node();
        let delim = self.elem.delim(*style_chain);
        let leftright = katex::LeftRightBuilder::default()
            .body([array].to_vec())
            .left(delim.open().unwrap().to_string())
            .right(delim.close().unwrap().to_string())
            .build().unwrap().into_node();
        Node::Node(leftright)
    }
}

pub struct TextConverter<'a> {
    pub elem: &'a typst::text::TextElem,
}

impl<'a> TextConverter<'a> {
    pub fn new(elem: &'a typst::text::TextElem) -> Self {
        Self {
            elem,
        }
    }

    pub fn convert(&mut self) -> Node {
        let text = &self.elem.text;
        if text.chars().count() == 1 {
            let name = text.chars().next().unwrap();
            self.convert_char(name, katex::Mode::Math)
        } else {
            self.convert_text(text)
        }
    }

    pub fn convert_text(&mut self, text: &str) -> Node {
        let body = text.chars().map(|name| katex::Symbol::get(katex::Mode::Text, name).create_node()).collect();

        let text = katex::TextBuilder::default()
            .body(body)
            .build().unwrap().into_node();
        Node::Node(text)
    }

    pub fn convert_char(&mut self, name: char, mode: katex::Mode) -> Node {
        match name {
            '≔' => symbol::define(),
            '≠' => symbol::neq(),
            _ => Node::Node(katex::Symbol::get(mode, name).create_node())
        }
    }
}
