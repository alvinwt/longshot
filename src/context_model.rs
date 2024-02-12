pub struct ContextModel {
    pub k: usize,
    pub alphabet: bio::alphabets::Alphabet,
    pub rank_transform: bio::alphabets::RankTransform
}

impl ContextModel {
    pub fn init(k: usize) -> ContextModel {
        let a =  bio::alphabets::Alphabet::new(b"ACGT");
        let rt = bio::alphabets::RankTransform::new(&a);

        ContextModel {
            k: k,
            alphabet: a,
            rank_transform: rt
        }
    }

    pub fn num_symbols(&self) -> usize {
        return self.alphabet.len();
    }

    pub fn num_kmers(&self) -> usize {
        let n_symbols = self.num_symbols();
        return n_symbols.pow(self.k as u32);
    }

    pub fn num_contexts(&self) -> usize {
        return self.num_kmers() * 2; // one set of k-mers for each strand
    }

    pub fn get_context_rank<'a>(&self, context: &[u8], is_reverse: bool) -> Option<usize> {
        assert!(context.len() == self.k);

        if !self.alphabet.is_word(context) {
            return None;
        }

        let base_rank = (is_reverse as usize) * self.num_kmers();

        // API converts an iterator into a vector of ranks
        // we only have one here  since len == k
        let mut ranks = self.rank_transform.qgrams(self.k as u32, context.iter());
        return Some(base_rank + ranks.next().unwrap()); 
    }

    pub fn get_base_rank(&self, base: char) -> u8 {
        self.rank_transform.get(base as u8)
    }
}

impl Clone for ContextModel {
    fn clone(&self) -> ContextModel {
        let bases = self.alphabet.symbols.into_iter().map(|x| x as u8).collect::<Vec::<u8>>();
        let a = bio::alphabets::Alphabet::new(bases);
        let rt = bio::alphabets::RankTransform::new(&a);
        
        ContextModel {
            k: self.k,
            alphabet: a,
            rank_transform: rt
        }
    }
}

