use anchor_lang::prelude::*;

declare_id!("EJSdaQmHVRBbnMuRcctwp7afchMHJ4evD5pZm6967yxw");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

//BKPQaVb4oek5YCRdjMKensPMASdUM7aaHGqnDNoDLZAs
//9ooFAd8MHJXVAK5Pt1gsStVF9rqpD7iqrRdcRxpFQH9k