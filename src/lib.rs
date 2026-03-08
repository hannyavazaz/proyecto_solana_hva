use anchor_lang::prelude::*;

declare_id!("7q2NN2fGt47wLd6Y3fjrJaNtfZMTW4Mv8GKB9dUamPQi");

#[program]
pub mod tienda_musica {
    use super::*;

    pub fn crear_tienda(ctx: Context<CrearTienda>, nombre: String, domicilio: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        tienda.owner = *ctx.accounts.usuario.key;
        tienda.nombre = nombre;
        tienda.domicilio = domicilio;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearTienda<'info> {
    #[account(init, payer = usuario, space = 8 + 32 + 64 + 64)]
    pub tienda: Account<'info, TiendaMusica>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TiendaMusica {
    pub owner: Pubkey,
    #[max_len(60)]
    pub nombre: String,
    #[max_len(60)]
    pub domicilio: String,
}

#[derive( AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug) ]
pub struct Album {
    #[max_len(60)]
    pub nombre: String,
    #[max_len(60)]
    pub artista: String,
    pub cant_canciones: u8,
}

pub struct Cancion {
    #[max_len(60)]
    pub nombre: String,
    pub duracion: u16,
    #[max_len(60)]
    pub genero: String,
}
