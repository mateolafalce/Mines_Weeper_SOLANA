use anchor_lang::prelude::*;

declare_id!("BhuUVWQqk5AaftS4TVkdLGYWCWqhCfQqYDhbUVtE4r9v");

#[program]
pub mod minersweeper {
    use super::*;

    pub fn create(ctx: Context<Create>,
        player: Pubkey,
        row1: Vec<u8>,
        row2: Vec<u8>,
        row3: Vec<u8>,
        row4: Vec<u8>,
        row5: Vec<u8>,
        row6: Vec<u8>,
        row7: Vec<u8>,
        row8: Vec<u8>
    ) -> Result<()> {
        let row_colums = &mut ctx.accounts.row_colums;
        row_colums.player = player;
        row_colums.row1 = row1;row_colums.row2 = row2;
        row_colums.row3 = row3;row_colums.row4 = row4;
        row_colums.row5 = row5;row_colums.row6 = row6;
        row_colums.row7 = row7;row_colums.row8 = row8;
        let mut f1 = row_colums.row1.clone();let mut f2 = row_colums.row2.clone();
        let mut f3 = row_colums.row3.clone();let mut f4 = row_colums.row4.clone();
        let mut f5 = row_colums.row5.clone();let mut f6 = row_colums.row6.clone();
        let mut f7 = row_colums.row7.clone();let mut f8 = row_colums.row8.clone();
//--------------------------------------------------------------------
        if row_colums.row1.len() != 8 {
            return Err(ErrorCode::ErrorSize.into())
        } 
        if row_colums.row2.len() != 8 {
            return Err(ErrorCode::ErrorSize.into())
        } 
        if row_colums.row3.len() != 8 {
            return Err(ErrorCode::ErrorSize.into())
        } 
        if row_colums.row4.len() != 8 {
            return Err(ErrorCode::ErrorSize.into())
        } 
        if row_colums.row5.len() != 8 {
            return Err(ErrorCode::ErrorSize.into())
        } 
        if row_colums.row6.len() != 8 {
            return Err(ErrorCode::ErrorSize.into())
        } 
        if row_colums.row7.len() != 8 {
            return Err(ErrorCode::ErrorSize.into())
        } 
        if row_colums.row8.len() != 8 {
            return Err(ErrorCode::ErrorSize.into())
        }
//--------------------------------------------------------------------
        if row_colums.row1[0] == 7 {
            f1[0] = 1;
            f1[1] = 2;
            f2[1] = 2;
        }
        if row_colums.row1[0] == 8 {
            f1[0] = 8;
        }
//--------------------------------------------------------------------
    if row_colums.row1[1] == 7 {
        f1[0] = 1;
        f1[1] = 2;
        f1[2] = 1;

        f2[1] = 2;
    }
    if row_colums.row1[1] == 8 {
        f1[1] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row1[2] == 7 {
        f1[2] = 1;
        f1[1] = 2;
        f1[3] = 1;

        f2[1] = 2;
        f2[3] = 1;
    }
    if row_colums.row1[2] == 8 {
        f1[2] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row1[3] == 7 {
        f1[3] = 1;
        f1[2] = 1;
        f1[4] = 2;

        f2[3] = 1;
        f2[4] = 2;
    }
    if row_colums.row1[3] == 8 {
        f1[3] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row1[4] == 7 {
        f1[4] = 2;
        f1[3] = 1;

        f2[4] = 2;
        f2[3] = 1;
    }
    if row_colums.row1[4] == 8 {
        f1[4] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row1[5] == 7 {
        msg!("YOU LOST THE BOMB EXPLODED YOU");
    }
    if row_colums.row1[5] == 8 {
        f1[5] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row1[6] == 7 {
        f1[6] = 2;
        f1[7] = 6;

        f2[6] = 3;
        f2[7] = 1;
    }
    if row_colums.row1[6] == 8 {
        f1[6] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row1[7] == 7 {
        f1[7] = 6;
        f1[6] = 2;

        f2[7] = 1;
        f2[6] = 3;
    }
    if row_colums.row1[7] == 8 {
        f1[7] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row2[0] == 7 {
        msg!("YOU LOST THE BOMB EXPLODED YOU");
    }
    if row_colums.row2[0] == 8 {
        f2[0] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row2[1] == 7 {
        f1[0] = 1;
        f1[1] = 2;
        f1[2] = 1;

        f2[1] = 2;

        f3[0] = 1;
        f3[1] = 2;
        f3[2] = 1;
    }
    if row_colums.row2[1] == 8 {
        f2[1] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row2[2] == 7 {
        msg!("YOU LOST THE BOMB EXPLODED YOU");
    }
    if row_colums.row2[2] == 8 {
        f2[2] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row2[3] == 7 {
        f1[2] = 1;
        f1[3] = 1;
        f1[4] = 2;

        f2[3] = 1;
        f2[4] = 2;

        f3[2] = 1;
        f3[3] = 1;
        f3[4] = 1;
    }
    if row_colums.row2[3] == 8 {
        f2[3] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row2[4] == 7 {
        f1[3] = 1;
        f1[4] = 2;

        f2[4] = 1;
        f2[4] = 2;

        f3[3] = 1;
        f3[4] = 1;
        f3[5] = 1;
    }
    if row_colums.row2[4] == 8 {
        f2[4] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row2[5] == 7 {
        msg!("YOU LOST THE BOMB EXPLODED YOU");
    }
    if row_colums.row2[5] == 8 {
        f2[5] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row2[6] == 7 {
        f1[7] = 6;
        f1[6] = 2;

        f2[6] = 3;
        f2[7] = 1;

        f3[6] = 3;
        f3[5] = 1;
    }
    if row_colums.row2[6] == 8 {
        f2[6] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row2[7] == 7 {
        f1[7] = 6;
        f1[6] = 2;

        f2[6] = 3;
        f2[7] = 1;

        f3[6] = 3;
    }
    if row_colums.row2[7] == 8 {
        f2[7] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row3[0] == 7 {
        f2[1] = 2;

        f3[0] = 1;
        f3[1] = 2;

        f4[0] = 6;
        f4[1] = 6;
    }
    if row_colums.row3[0] == 8 {
        f3[0] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row3[1] == 7 {
        f2[1] = 2;

        f3[0] = 1;
        f3[1] = 2;
        f3[2] = 1;

        f4[0] = 6;
        f4[1] = 6;
        f4[2] = 6;
    }
    if row_colums.row3[1] == 8 {
        f3[1] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row3[2] == 7 {
        f2[1] = 2;
        f2[3] = 1;

        f3[1] = 2;
        f3[2] = 1;
        f3[3] = 1;

        f4[1] = 6;
        f4[2] = 6;
        f4[3] = 6;
    }
    if row_colums.row3[2] == 8 {
        f3[2] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row3[3] == 7 {
        f2[4] = 2;
        f2[3] = 1;

        f3[4] = 1;
        f3[2] = 1;
        f3[3] = 1;

        f4[4] = 6;
        f4[2] = 6;
        f4[3] = 6;
    }
    if row_colums.row3[3] == 8 {
        f3[3] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row3[4] == 7 {
        f2[4] = 2;
        f2[3] = 1;

        f3[4] = 1;
        f3[5] = 1;
        f3[3] = 1;

        f4[4] = 6;
        f4[5] = 6;
        f4[3] = 6;
    }
    if row_colums.row3[4] == 8 {
        f3[4] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row3[5] == 7 {
        f2[4] = 2;
        f2[6] = 3;

        f3[4] = 1;
        f3[5] = 1;
        f3[6] = 3;

        f4[4] = 6;
        f4[5] = 6;
        f4[6] = 2;
    }
    if row_colums.row3[5] == 8 {
        f3[5] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row3[6] == 7 {
        f2[7] = 1;
        f2[6] = 3;

        f3[5] = 1;
        f3[6] = 3;

        f4[5] = 6;
        f4[6] = 2;
    }
    if row_colums.row3[6] == 8 {
        f3[6] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row3[7] == 7 {
        msg!("YOU LOST THE BOMB EXPLODED YOU");
    }
    if row_colums.row3[7] == 8 {
        f3[7] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row4[0] == 7 {
        f3[0] = 1;
        f3[1] = 2;

        f4[0] = 6;
        f4[1] = 6;

        f5[0] = 6;
        f5[1] = 1;
    }
    if row_colums.row4[0] == 8 {
        f4[0] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row4[1] == 7 {
        f3[0] = 1;
        f3[1] = 2;
        f3[2] = 1;

        f4[0] = 6;
        f4[1] = 6;
        f4[2] = 6;

        f5[0] = 6;
        f5[1] = 1;
        f5[2] = 1;
    }
    if row_colums.row4[1] == 8 {
        f4[1] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row4[2] == 7 {
        f3[1] = 2;
        f3[2] = 1;
        f3[3] = 1;

        f4[1] = 6;
        f4[2] = 6;
        f4[3] = 6;

        f5[1] = 1;
        f5[2] = 1;
        f5[3] = 1;
    }
    if row_colums.row4[2] == 8 {
        f4[2] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row4[3] == 7 {
        f3[4] = 1;
        f3[2] = 1;
        f3[3] = 1;

        f4[4] = 6;
        f4[2] = 6;
        f4[3] = 6;

        f5[4] = 6;
        f5[2] = 1;
        f5[3] = 1;
    }
    if row_colums.row4[3] == 8 {
        f4[3] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row4[4] == 7 {
        f3[4] = 1;
        f3[5] = 1;
        f3[3] = 1;

        f4[4] = 6;
        f4[5] = 6;
        f4[3] = 6;

        f5[4] = 6;
        f5[5] = 6;
        f5[3] = 1;
    }
    if row_colums.row4[4] == 8 {
        f4[4] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row4[5] == 7 {
        f3[4] = 1;
        f3[5] = 1;
        f3[6] = 3;

        f4[4] = 6;
        f4[5] = 6;
        f4[6] = 2;

        f5[4] = 6;
        f5[5] = 6;
        f5[6] = 2;
    }
    if row_colums.row4[5] == 8 {
        f4[5] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row4[6] == 7 {
        f3[5] = 1;
        f3[6] = 3;

        f4[5] = 6;
        f4[6] = 2;

        f5[7] = 6;
        f5[5] = 6;
        f5[6] = 2;
    }
    if row_colums.row4[6] == 8 {
        f4[6] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row4[7] == 7 {
        msg!("YOU LOST THE BOMB EXPLODED YOU");
    }
    if row_colums.row4[7] == 8 {
        f4[7] = 8;
    }
//--------------------------------------------------------------------
    if row_colums.row5[0] == 7 {
        f4[0] = 6;
        f4[1] = 2;

        f5[0] = 6;
        f5[1] = 6;

        f6[0] = 6;
        f6[1] = 2;
    }
    if row_colums.row5[0] == 8 {
        f5[6] = 8;
    }


        msg!("{:?}", f1);
        msg!("{:?}", f2);
        msg!("{:?}", f3);
        msg!("{:?}", f4);
        msg!("{:?}", f5);
        msg!("{:?}", f6);
        msg!("{:?}", f7);
        msg!("{:?}", f8);
        msg!("Id: {}", row_colums.player);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = Minersweeper::LEN)]
    pub row_colums: Account<'info, Minersweeper>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Minersweeper {
    player: Pubkey,
    row1: Vec<u8>,
    row2: Vec<u8>,
    row3: Vec<u8>,
    row4: Vec<u8>,
    row5: Vec<u8>,
    row6: Vec<u8>,
    row7: Vec<u8>,
    row8: Vec<u8>
}

impl Minersweeper {
    const LEN: usize = DISCRIMINATOR 
    + PUBKEY
    + PREFIX
    + VECU8;
}

const DISCRIMINATOR: usize = 8;
const PUBKEY: usize = 32;
const PREFIX: usize = 4 * 8;
const VECU8: usize = 8 * 8;


#[error_code]
pub enum ErrorCode {
    #[msg("The vector must have a size of 8")]
    ErrorSize,
}
