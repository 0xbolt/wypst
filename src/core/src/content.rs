use crate::node::*;
use typst::foundations::{Content, StyleChain};

pub trait ContentVisitor {
    fn visit_equation(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_sequence(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_text(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_space(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_lr(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_attach(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_h(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_linebreak(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_align_point(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_frac(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_vec(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_mat(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_op(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_cases(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_overbracket(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_underbracket(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_overbrace(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_underbrace(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_overline(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_underline(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_root(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_mid(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_binom(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_class(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_cancel(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_limits(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_scripts(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_primes(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_accent(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_symbol(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
    fn visit_styled(&mut self, content: &Content, style_chain: &StyleChain) -> Node;
}

pub trait ContentType {
    fn is_equation(&self) -> bool;
    fn is_space(&self) -> bool;
    fn is_text(&self) -> bool;
    fn is_lr(&self) -> bool;
    fn is_attach(&self) -> bool;
    fn is_h(&self) -> bool;
    fn is_linebreak(&self) -> bool;
    fn is_align_point(&self) -> bool;
    fn is_frac(&self) -> bool;
    fn is_vec(&self) -> bool;
    fn is_mat(&self) -> bool;
    fn is_op(&self) -> bool;
    fn is_cases(&self) -> bool;
    fn is_overbracket(&self) -> bool;
    fn is_underbracket(&self) -> bool;
    fn is_overbrace(&self) -> bool;
    fn is_underbrace(&self) -> bool;
    fn is_overline(&self) -> bool;
    fn is_underline(&self) -> bool;
    fn is_root(&self) -> bool;
    fn is_mid(&self) -> bool;
    fn is_binom(&self) -> bool;
    fn is_class(&self) -> bool;
    fn is_cancel(&self) -> bool;
    fn is_limits(&self) -> bool;
    fn is_scripts(&self) -> bool;
    fn is_primes(&self) -> bool;
    fn is_accent(&self) -> bool;
    fn is_sequence(&self) -> bool;
    fn is_symbol(&self) -> bool;
    fn is_styled(&self) -> bool;

    fn to_equation(&self) -> &typst::math::EquationElem;
    fn to_space(&self) -> &typst::text::SpaceElem;
    fn to_text(&self) -> &typst::text::TextElem;
    fn to_lr(&self) -> &typst::math::LrElem;
    fn to_attach(&self) -> &typst::math::AttachElem;
    fn to_h(&self) -> &typst::layout::HElem;
    fn to_linebreak(&self) -> &typst::text::LinebreakElem;
    fn to_align_point(&self) -> &typst::math::AlignPointElem;
    fn to_frac(&self) -> &typst::math::FracElem;
    fn to_vec(&self) -> &typst::math::VecElem;
    fn to_mat(&self) -> &typst::math::MatElem;
    fn to_op(&self) -> &typst::math::OpElem;
    fn to_cases(&self) -> &typst::math::CasesElem;
    fn to_overbracket(&self) -> &typst::math::OverbracketElem;
    fn to_underbracket(&self) -> &typst::math::UnderbracketElem;
    fn to_overbrace(&self) -> &typst::math::OverbraceElem;
    fn to_underbrace(&self) -> &typst::math::UnderbraceElem;
    fn to_overline(&self) -> &typst::math::OverlineElem;
    fn to_underline(&self) -> &typst::math::UnderlineElem;
    fn to_root(&self) -> &typst::math::RootElem;
    fn to_mid(&self) -> &typst::math::MidElem;
    fn to_binom(&self) -> &typst::math::BinomElem;
    fn to_class(&self) -> &typst::math::ClassElem;
    fn to_cancel(&self) -> &typst::math::CancelElem;
    fn to_limits(&self) -> &typst::math::LimitsElem;
    fn to_scripts(&self) -> &typst::math::ScriptsElem;
    fn to_primes(&self) -> &typst::math::PrimesElem;
    fn to_accent(&self) -> &typst::math::AccentElem;
    fn to_sequence(&self) -> Option<impl Iterator<Item = &Content>>;
    fn to_symbol(&self) -> &typst::foundations::SymbolElem;
    fn to_styled(&self) -> &typst::foundations::StyledElem;
}

impl ContentType for Content {
    fn is_equation(&self) -> bool {
        self.is::<typst::math::EquationElem>()
    }
    fn is_space(&self) -> bool {
        self.is::<typst::text::SpaceElem>()
    }
    fn is_text(&self) -> bool {
        self.is::<typst::text::TextElem>()
    }
    fn is_lr(&self) -> bool {
        self.is::<typst::math::LrElem>()
    }
    fn is_attach(&self) -> bool {
        self.is::<typst::math::AttachElem>()
    }
    fn is_h(&self) -> bool {
        self.is::<typst::layout::HElem>()
    }
    fn is_linebreak(&self) -> bool {
        self.is::<typst::text::LinebreakElem>()
    }
    fn is_align_point(&self) -> bool {
        self.is::<typst::math::AlignPointElem>()
    }
    fn is_frac(&self) -> bool {
        self.is::<typst::math::FracElem>()
    }
    fn is_vec(&self) -> bool {
        self.is::<typst::math::VecElem>()
    }
    fn is_mat(&self) -> bool {
        self.is::<typst::math::MatElem>()
    }
    fn is_op(&self) -> bool {
        self.is::<typst::math::OpElem>()
    }
    fn is_cases(&self) -> bool {
        self.is::<typst::math::CasesElem>()
    }
    fn is_overbracket(&self) -> bool {
        self.is::<typst::math::OverbracketElem>()
    }
    fn is_underbracket(&self) -> bool {
        self.is::<typst::math::UnderbracketElem>()
    }
    fn is_overbrace(&self) -> bool {
        self.is::<typst::math::OverbraceElem>()
    }
    fn is_underbrace(&self) -> bool {
        self.is::<typst::math::UnderbraceElem>()
    }
    fn is_overline(&self) -> bool {
        self.is::<typst::math::OverlineElem>()
    }
    fn is_underline(&self) -> bool {
        self.is::<typst::math::UnderlineElem>()
    }
    fn is_root(&self) -> bool {
        self.is::<typst::math::RootElem>()
    }
    fn is_mid(&self) -> bool {
        self.is::<typst::math::MidElem>()
    }
    fn is_binom(&self) -> bool {
        self.is::<typst::math::BinomElem>()
    }
    fn is_class(&self) -> bool {
        self.is::<typst::math::ClassElem>()
    }
    fn is_cancel(&self) -> bool {
        self.is::<typst::math::CancelElem>()
    }
    fn is_limits(&self) -> bool {
        self.is::<typst::math::LimitsElem>()
    }
    fn is_scripts(&self) -> bool {
        self.is::<typst::math::ScriptsElem>()
    }
    fn is_primes(&self) -> bool {
        self.is::<typst::math::PrimesElem>()
    }
    fn is_accent(&self) -> bool {
        self.is::<typst::math::AccentElem>()
    }
    fn is_sequence(&self) -> bool {
        self.is::<typst::foundations::SequenceElem>()
    }
    fn is_symbol(&self) -> bool {
        self.is::<typst::foundations::SymbolElem>()
    }
    fn is_styled(&self) -> bool {
        self.is::<typst::foundations::StyledElem>()
    }

    fn to_equation(&self) -> &typst::math::EquationElem {
        self.to_packed::<typst::math::EquationElem>().unwrap()
    }
    fn to_space(&self) -> &typst::text::SpaceElem {
        self.to_packed::<typst::text::SpaceElem>().unwrap()
    }
    fn to_text(&self) -> &typst::text::TextElem {
        self.to_packed::<typst::text::TextElem>().unwrap()
    }
    fn to_lr(&self) -> &typst::math::LrElem {
        self.to_packed::<typst::math::LrElem>().unwrap()
    }
    fn to_attach(&self) -> &typst::math::AttachElem {
        self.to_packed::<typst::math::AttachElem>().unwrap()
    }
    fn to_h(&self) -> &typst::layout::HElem {
        self.to_packed::<typst::layout::HElem>().unwrap()
    }
    fn to_linebreak(&self) -> &typst::text::LinebreakElem {
        self.to_packed::<typst::text::LinebreakElem>().unwrap()
    }
    fn to_align_point(&self) -> &typst::math::AlignPointElem {
        self.to_packed::<typst::math::AlignPointElem>().unwrap()
    }
    fn to_frac(&self) -> &typst::math::FracElem {
        self.to_packed::<typst::math::FracElem>().unwrap()
    }
    fn to_vec(&self) -> &typst::math::VecElem {
        self.to_packed::<typst::math::VecElem>().unwrap()
    }
    fn to_mat(&self) -> &typst::math::MatElem {
        self.to_packed::<typst::math::MatElem>().unwrap()
    }
    fn to_op(&self) -> &typst::math::OpElem {
        self.to_packed::<typst::math::OpElem>().unwrap()
    }
    fn to_cases(&self) -> &typst::math::CasesElem {
        self.to_packed::<typst::math::CasesElem>().unwrap()
    }
    fn to_overbracket(&self) -> &typst::math::OverbracketElem {
        self.to_packed::<typst::math::OverbracketElem>().unwrap()
    }
    fn to_underbracket(&self) -> &typst::math::UnderbracketElem {
        self.to_packed::<typst::math::UnderbracketElem>().unwrap()
    }
    fn to_overbrace(&self) -> &typst::math::OverbraceElem {
        self.to_packed::<typst::math::OverbraceElem>().unwrap()
    }
    fn to_underbrace(&self) -> &typst::math::UnderbraceElem {
        self.to_packed::<typst::math::UnderbraceElem>().unwrap()
    }
    fn to_overline(&self) -> &typst::math::OverlineElem {
        self.to_packed::<typst::math::OverlineElem>().unwrap()
    }
    fn to_underline(&self) -> &typst::math::UnderlineElem {
        self.to_packed::<typst::math::UnderlineElem>().unwrap()
    }
    fn to_root(&self) -> &typst::math::RootElem {
        self.to_packed::<typst::math::RootElem>().unwrap()
    }
    fn to_mid(&self) -> &typst::math::MidElem {
        self.to_packed::<typst::math::MidElem>().unwrap()
    }
    fn to_binom(&self) -> &typst::math::BinomElem {
        self.to_packed::<typst::math::BinomElem>().unwrap()
    }
    fn to_class(&self) -> &typst::math::ClassElem {
        self.to_packed::<typst::math::ClassElem>().unwrap()
    }
    fn to_cancel(&self) -> &typst::math::CancelElem {
        self.to_packed::<typst::math::CancelElem>().unwrap()
    }
    fn to_limits(&self) -> &typst::math::LimitsElem {
        self.to_packed::<typst::math::LimitsElem>().unwrap()
    }
    fn to_scripts(&self) -> &typst::math::ScriptsElem {
        self.to_packed::<typst::math::ScriptsElem>().unwrap()
    }
    fn to_primes(&self) -> &typst::math::PrimesElem {
        self.to_packed::<typst::math::PrimesElem>().unwrap()
    }
    fn to_accent(&self) -> &typst::math::AccentElem {
        self.to_packed::<typst::math::AccentElem>().unwrap()
    }
    fn to_sequence(&self) -> Option<impl Iterator<Item = &Content>> {
        let sequence = self.to_packed::<typst::foundations::SequenceElem>()?;

        Some(sequence.children.iter())
    }
    fn to_symbol(&self) -> &typst::foundations::SymbolElem {
        self.to_packed::<typst::foundations::SymbolElem>().unwrap()
    }
    fn to_styled(&self) -> &typst::foundations::StyledElem {
        self.to_packed::<typst::foundations::StyledElem>().unwrap()
    }
}

pub trait ContentExt {
    fn accept(&self, visitor: &mut dyn ContentVisitor, style_chain: &StyleChain) -> Node;
}

impl ContentExt for Content {
    fn accept(&self, visitor: &mut dyn ContentVisitor, style_chain: &StyleChain) -> Node {
        match self {
            _ if self.is_equation() => visitor.visit_equation(self, style_chain),
            _ if self.is_space() => visitor.visit_space(self, style_chain),
            _ if self.is_text() => visitor.visit_text(self, style_chain),
            _ if self.is_lr() => visitor.visit_lr(self, style_chain),
            _ if self.is_attach() => visitor.visit_attach(self, style_chain),
            _ if self.is_h() => visitor.visit_h(self, style_chain),
            _ if self.is_linebreak() => visitor.visit_linebreak(self, style_chain),
            _ if self.is_align_point() => visitor.visit_align_point(self, style_chain),
            _ if self.is_frac() => visitor.visit_frac(self, style_chain),
            _ if self.is_vec() => visitor.visit_vec(self, style_chain),
            _ if self.is_mat() => visitor.visit_mat(self, style_chain),
            _ if self.is_op() => visitor.visit_op(self, style_chain),
            _ if self.is_cases() => visitor.visit_cases(self, style_chain),
            _ if self.is_sequence() => visitor.visit_sequence(self, style_chain),
            _ if self.is_binom() => visitor.visit_binom(self, style_chain),
            _ if self.is_cancel() => visitor.visit_cancel(self, style_chain),
            _ if self.is_overbracket() => visitor.visit_overbracket(self, style_chain),
            _ if self.is_underbracket() => visitor.visit_underbracket(self, style_chain),
            _ if self.is_overline() => visitor.visit_overline(self, style_chain),
            _ if self.is_underline() => visitor.visit_underline(self, style_chain),
            _ if self.is_overbrace() => visitor.visit_overbrace(self, style_chain),
            _ if self.is_underbrace() => visitor.visit_underbrace(self, style_chain),
            _ if self.is_root() => visitor.visit_root(self, style_chain),
            _ if self.is_mid() => visitor.visit_mid(self, style_chain),
            _ if self.is_class() => visitor.visit_class(self, style_chain),
            _ if self.is_limits() => visitor.visit_limits(self, style_chain),
            _ if self.is_scripts() => visitor.visit_scripts(self, style_chain),
            _ if self.is_primes() => visitor.visit_primes(self, style_chain),
            _ if self.is_accent() => visitor.visit_accent(self, style_chain),
            _ if self.is_symbol() => visitor.visit_symbol(self, style_chain),
            _ if self.is_styled() => visitor.visit_styled(self, style_chain),
            // This ignores the elements and gracefully fails;
            // Must add results and errors eventually.
            _ => Node::Array(vec![]),
            // _ => panic!("Content element `{:#?}` not implemented yet.", self),
        }
    }
}
