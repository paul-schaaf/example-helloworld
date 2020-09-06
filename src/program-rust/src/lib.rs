#![cfg(feature = "program")]

use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    info,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
fn process_instruction<'a>(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &'a [AccountInfo<'a>], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    info!("Helloworld Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        info!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let new_fen_bytes = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".as_bytes();
    let mut data = account.try_borrow_mut_data()?;
    for x in data.iter_mut() {
        *x = 0;
    }

    for x in 0..new_fen_bytes.len() {
        data[x] = new_fen_bytes[x];
    }

    info!("Hello!");

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_sdk::clock::Epoch;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u64>()];
        LittleEndian::write_u64(&mut data, 0);
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(LittleEndian::read_u64(&accounts[0].data.borrow()), 0);
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(LittleEndian::read_u64(&accounts[0].data.borrow()), 1);
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(LittleEndian::read_u64(&accounts[0].data.borrow()), 2);
    }
}

// Required to support info! in tests
#[cfg(not(target_arch = "bpf"))]
solana_sdk::program_stubs!();
