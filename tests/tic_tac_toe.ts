import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { TicTacToe } from '../target/types/tic_tac_toe'
import { expect } from 'chai'

describe('tic_tac_toe', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.TicTacToe as Program<TicTacToe>

  const initiator = anchor.web3.Keypair.generate()
  let player = anchor.web3.Keypair.generate()

  before(async () => {
    await anchor
      .getProvider()
      .connection.confirmTransaction(
        await anchor
          .getProvider()
          .connection.requestAirdrop(initiator.publicKey, 10000000000),
        'confirmed'
      )
    await anchor
      .getProvider()
      .connection.confirmTransaction(
        await anchor
          .getProvider()
          .connection.requestAirdrop(player.publicKey, 10000000000),
        'confirmed'
      )
  })

  it('Initialize game', async () => {
    const [game] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('game'), initiator.publicKey.toBuffer()],
      program.programId
    )
    const tx = await program.methods
      .initializeGame()
      .accounts({
        initiator: initiator.publicKey,
      })
      .signers([initiator])
      .rpc()
    const gameData = await program.account.game.fetch(game)
    expect(gameData.initiator).to.eql(initiator.publicKey)
  })

  it('Join game', async () => {
    const [game] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('game'), initiator.publicKey.toBuffer()],
      program.programId
    )
    const tx = await program.methods
      .joinGame(initiator.publicKey)
      .accounts({
        player: player.publicKey,
      })
      .signers([player])
      .rpc()
    const gameData = await program.account.game.fetch(game)
    expect(gameData.playerO.toBase58()).to.oneOf([
      initiator.publicKey.toBase58(),
      player.publicKey.toBase58(),
    ])
  })

  it("Can't join already started game", async () => {
    const [game] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('game'), initiator.publicKey.toBuffer()],
      program.programId
    )
    try {
      await program.methods
        .joinGame(initiator.publicKey)
        .accounts({
          player: player.publicKey,
        })
        .signers([player])
        .rpc()
    } catch (error) {
      expect(error.error.errorMessage).to.be.eql('GameAlreadyStarted')
    }
  })

  it('Make move', async () => {
    const [game] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('game'), initiator.publicKey.toBuffer()],
      program.programId
    )
    let gameData = await program.account.game.fetch(game)
    let turn = gameData.playerX
    const tx = await program.methods
      .makeMove(initiator.publicKey, 0, 0)
      .accounts({
        player: turn,
      })
      .signers([
        turn.toBase58() === initiator.publicKey.toBase58() ? initiator : player,
      ])
      .rpc()
    gameData = await program.account.game.fetch(game)
    expect(gameData.board[0][0].x).to.be.eql({})
  })

  it("Can't make move if it's already taken", async () => {
    const [game] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('game'), initiator.publicKey.toBuffer()],
      program.programId
    )
    let gameData = await program.account.game.fetch(game)
    let turn = gameData.playerO
    try {
      await program.methods
        .makeMove(initiator.publicKey, 0, 0)
        .accounts({
          player: turn,
        })
        .signers([
          turn.toBase58() === initiator.publicKey.toBase58()
            ? initiator
            : player,
        ])
        .rpc()
    } catch (error) {
      expect(error.error.errorMessage).to.be.eql('CellAlreadyTaken')
    }
  })
})
