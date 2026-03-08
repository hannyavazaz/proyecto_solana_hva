use anchor_lang::prelude::*;

declare_id!("7q2NN2fGt47wLd6Y3fjrJaNtfZMTW4Mv8GKB9dUamPQi");

#[program]
pub mod tienda_musica {
    use super::*;

    pub fn crear_tienda(
        context: Context<NuevaTienda>,
        nombre: String
    ) -> Result<()> {

        let owner_id = context.accounts.owner.key();

        let album: Vec<Album> = Vec::new();
        let cancion: Vec<Cancion> = Vec::new();

        context.accounts.tiendamusica.set_inner(TiendaMusica {
            owner: owner_id,
            nombre,
            album,
            cancion,
        });

        Ok(())
    }

    pub fn agregar_album(
        context: Context<NuevoAlbum>,
        nombre: String,
        artista: String,
        cant_canciones: u8
    ) -> Result<()> {

        let album = Album {
            nombre,
            artista,
            cant_canciones
        };

        context.accounts.tiendamusica.album.push(album);

        Ok(())
    }

    pub fn agregar_cancion(
        context: Context<NuevaCancion>,
        nombre: String,
        duracion: u16,
        genero: String
    ) -> Result<()> {

        let cancion = Cancion {
            nombre,
            duracion,
            genero
        };

        context.accounts.tiendamusica.cancion.push(cancion);

        Ok(())
    }

    pub fn ver_album(context: Context<NuevoAlbum>) -> Result<()> {
        msg!("Albums: {:#?}", context.accounts.tiendamusica.album);
        Ok(())
    }

    pub fn ver_cancion(context: Context<NuevaCancion>) -> Result<()> {
        msg!("Canciones: {:#?}", context.accounts.tiendamusica.cancion);
        Ok(())
    }

    pub fn eliminar_album(context: Context<NuevoAlbum>, nombre: String) -> Result<()> {

    let album = &mut context.accounts.tiendamusica.album;

    for i in 0..album.len() {

        if album[i].nombre == nombre {

            album.remove(i);

            msg!("Album eliminado");
            return Ok(())
        }
    }

    Err(Errores::AlbumNoExiste.into())
}
   
    pub fn eliminar_cancion(context: Context<NuevaCancion>, nombre: String) -> Result<()> {

    let cancion = &mut context.accounts.tiendamusica.cancion;

    for i in 0..cancion.len() {

        if cancion[i].nombre == nombre {

            cancion.remove(i);

            msg!("Cancion eliminada");
            return Ok(())
        }
    }

    Err(Errores::CancionNoExiste.into())
}

    pub fn modificar_album(context: Context<NuevoAlbum>, nombre_actual: String, nuevo_nombre: String, nuevo_artista: String) -> Result<()> {

    let album = &mut context.accounts.tiendamusica.album;

    for i in 0..album.len() {
        if album[i].nombre == nombre_actual {

            album[i].nombre = nuevo_nombre;
            album[i].artista = nuevo_artista;

            msg!("Album modificado");
            return Ok(())
        }
    }

    Err(Errores::AlbumNoExiste.into())
}

pub fn modificar_cancion(
    context: Context<NuevaCancion>,
    nombre_actual: String,
    nuevo_nombre: String,
    nuevo_genero: String
) -> Result<()> {

    let cancion = &mut context.accounts.tiendamusica.cancion;

    for i in 0..cancion.len() {
        if cancion[i].nombre == nombre_actual {

            cancion[i].nombre = nuevo_nombre;
            cancion[i].genero = nuevo_genero;

            msg!("Cancion modificada");
            return Ok(())
        }
    }

    Err(Errores::CancionNoExiste.into())
}

    
}

#[error_code]
pub enum Errores{
    #[msg("Error. No eres el propietario de la cuenta")]
    NoEresElOwner,
    #[msg("Error. El album no existe")]
    AlbumNoExiste,
    #[msg("Error. Esta cancion no existe")]
    CancionNoExiste, 
}


#[account]
#[derive(InitSpace)]
pub struct TiendaMusica {

    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(10)]
    pub album: Vec<Album>,

    #[max_len(50)]
    pub cancion: Vec<Cancion>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Album {

    #[max_len(60)]
    pub nombre: String,

    #[max_len(60)]
    pub artista: String,

    pub cant_canciones: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Cancion {

    #[max_len(60)]
    pub nombre: String,

    pub duracion: u16,

    #[max_len(60)]
    pub genero: String,
}

#[derive(Accounts)]
pub struct NuevaTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + TiendaMusica::INIT_SPACE,
        seeds = [b"tiendamusica", owner.key().as_ref()],
        bump
    )]
    pub tiendamusica: Account<'info, TiendaMusica>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoAlbum<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub tiendamusica: Account<'info, TiendaMusica>,
}

#[derive(Accounts)]
pub struct NuevaCancion<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub tiendamusica: Account<'info, TiendaMusica>,
}
