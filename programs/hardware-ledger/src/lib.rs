use anchor_lang::prelude::*;

declare_id!("6yGayNAcQgEymcGYhCR2KeDa6rWQgT7AK351pCtRHqFc");

#[program]
pub mod hardware_ledger {
    use super::*;

    pub fn crear_workstation_db(
        ctx: Context<CrearWorkstationDB>,
        nombre_equipo: String,
    ) -> Result<()> {
        let workstation = &mut ctx.accounts.workstation_db;

        require!(
            nombre_equipo.len() > 0 && nombre_equipo.len() <= 30,
            ErrorCode::TextoInvalido
        );

        workstation.nombre_equipo = nombre_equipo;
        workstation.componentes = Vec::new();

        msg!(
            "Workstation '{}' registrada con éxito.",
            workstation.nombre_equipo
        );
        Ok(())
    }

    pub fn agregar_componente(
        ctx: Context<AgregarComponente>,
        modelo: String,
        tipo: String,
        metrica_principal: u32,
        estado_calidad: u8,
    ) -> Result<()> {
        let workstation = &mut ctx.accounts.workstation_db;
        let componente = &mut ctx.accounts.componente;

        require!(
            modelo.len() > 0 && modelo.len() <= 40,
            ErrorCode::TextoInvalido
        );
        require!(tipo.len() > 0 && tipo.len() <= 20, ErrorCode::TextoInvalido);

        componente.modelo = modelo;
        componente.tipo = tipo;
        componente.metrica_principal = metrica_principal;
        componente.estado_calidad = estado_calidad;

        workstation.componentes.push(componente.key());

        msg!(
            "Componente '{}' agregado a la Workstation.",
            componente.modelo
        );
        Ok(())
    }

    pub fn actualizar_componente(
        ctx: Context<ActualizarComponente>, 
        modelo: String, 
        nueva_metrica: u32, 
        nuevo_estado: u8
    ) -> Result<()> {
        let componente = &mut ctx.accounts.componente;
        
        componente.metrica_principal = nueva_metrica;
        componente.estado_calidad = nuevo_estado;

        msg!("Componente '{}' actualizado. Nueva métrica: {}", componente.modelo, nueva_metrica);
        Ok(())
    }

    pub fn eliminar_componente(
        _ctx: Context<EliminarComponente>, 
        _modelo: String 
    ) -> Result<()> {
        msg!("Componente eliminado. Renta recuperada con éxito.");
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("El texto proporcionado esta vacio o excede el límite permitido.")]
    TextoInvalido,
}

#[derive(Accounts)]
pub struct CrearWorkstationDB<'info> {
    #[account(
        init,
        payer = usuario,
        space = 8 + WorkstationDB::INIT_SPACE,
        seeds = [b"workstation", usuario.key().as_ref()],
        bump
    )]
    pub workstation_db: Account<'info, WorkstationDB>,

    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(modelo: String)]
pub struct AgregarComponente<'info> {
    #[account(mut)]
    pub workstation_db: Account<'info, WorkstationDB>,

    #[account(
        init,
        payer = usuario,
        space = 8 + Componente::INIT_SPACE,
        seeds = [b"componente", usuario.key().as_ref(), modelo.as_bytes()],
        bump
    )]
    pub componente: Account<'info, Componente>,

    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(modelo: String)]
pub struct ActualizarComponente<'info> {
    #[account(
        mut,
        seeds = [b"componente", usuario.key().as_ref(), modelo.as_bytes()],
        bump
    )]
    pub componente: Account<'info, Componente>,

    #[account(mut)]
    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(modelo: String)]
pub struct EliminarComponente<'info> {
    #[account(
        mut,
        close = usuario,
        seeds = [b"componente", usuario.key().as_ref(), modelo.as_bytes()],
        bump
    )]
    pub componente: Account<'info, Componente>,

    #[account(mut)]
    pub usuario: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct WorkstationDB {
    #[max_len(30)]
    pub nombre_equipo: String,

    #[max_len(10)]
    pub componentes: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Componente {
    #[max_len(20)]
    pub tipo: String,

    #[max_len(40)]
    pub modelo: String,

    pub metrica_principal: u32,

    pub estado_calidad: u8,
}