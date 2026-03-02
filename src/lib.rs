use anchor_lang::prelude::*;

declare_id!("Pet111111111111111111111111111111111111111");

#[program]
pub mod registro_mascotas {
    use super::*;

    pub fn crear_dueno(ctx: Context<CrearDueno>, nombre: String) -> Result<()> {
        let dueno = &mut ctx.accounts.dueno;
        dueno.owner = ctx.accounts.owner.key();
        dueno.nombre = nombre;
        dueno.mascotas = Vec::new();
        Ok(())
    }

    pub fn registrar_mascota(
        ctx: Context<RegistrarMascota>,
        nombre: String,
        especie: String,
        edad: u8,
    ) -> Result<()> {
        let mascota = &mut ctx.accounts.mascota;
        let dueno = &mut ctx.accounts.dueno;

        mascota.dueno = dueno.nombre.clone();
        mascota.nombre = nombre;
        mascota.especie = especie;
        mascota.edad = edad;
        mascota.activa = true;

        dueno.mascotas.push(ctx.accounts.mascota.key());

        Ok(())
    }

    pub fn eliminar_mascota(_ctx: Context<EliminarMascota>) -> Result<()> {
        Ok(())
    }

    pub fn alternar_estado(ctx: Context<ModificarMascota>) -> Result<()> {
        let mascota = &mut ctx.accounts.mascota;
        mascota.activa = !mascota.activa;
        Ok(())
    }

    pub fn actualizar_edad(ctx: Context<ModificarMascota>, nueva_edad: u8) -> Result<()> {
        let mascota = &mut ctx.accounts.mascota;
        mascota.edad = nueva_edad;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct CrearDueno<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + 50 + 4 + (32 * 10),
        seeds = [b"dueno", nombre.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub dueno: Account<'info, Dueno>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct RegistrarMascota<'info> {
    #[account(
        mut,
        has_one = owner
    )]
    pub dueno: Account<'info, Dueno>,

    #[account(
        init,
        payer = owner,
        space = 8 + 4 + 50 + 4 + 50 + 1 + 1,
        seeds = [b"mascota", nombre.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub mascota: Account<'info, Mascota>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EliminarMascota<'info> {
    #[account(
        mut,
        close = owner
    )]
    pub mascota: Account<'info, Mascota>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ModificarMascota<'info> {
    #[account(mut)]
    pub mascota: Account<'info, Mascota>,

    pub owner: Signer<'info>,
}

#[account]
pub struct Dueno {
    pub owner: Pubkey,
    pub nombre: String,
    pub mascotas: Vec<Pubkey>,
}

#[account]
pub struct Mascota {
    pub dueno: String,
    pub nombre: String,
    pub especie: String,
    pub edad: u8,
    pub activa: bool,
}
