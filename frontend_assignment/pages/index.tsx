import detectEthereumProvider from "@metamask/detect-provider"
import { Strategy, ZkIdentity } from "@zk-kit/identity"
import { generateMerkleProof, Semaphore } from "@zk-kit/protocols"
import { providers } from "ethers"
import Head from "next/head"
import React from "react"
import styles from "../styles/Home.module.css"
import { useForm } from 'react-hook-form';
import * as yup from 'yup';
import { yupResolver } from '@hookform/resolvers/yup';


const personSchema = yup.object({
    firstName: yup.string().defined(),
    lastName: yup.string().default('').nullable(),
    email: yup.string().nullable().email(),
    age: yup.number().nullable().min(0).max(120),
});

interface Person extends yup.InferType<typeof personSchema> {
}


export default function Home() {

    const [logs, setLogs] = React.useState("Connect your wallet and greet!")
    const { register, handleSubmit, formState: { errors } } = useForm<Person>(yupResolver(personSchema));
    const onSubmit = handleSubmit((data: any) => console.log(data));
    const [newGreeter, setNewGreeter] = React.useState<ZkIdentity | null>(null)
    async function greet() {
        setLogs("Creating your Semaphore identity...")
        const provider = (await detectEthereumProvider()) as any

        await provider.request({ method: "eth_requestAccounts" })

        const ethersProvider = new providers.Web3Provider(provider)
        const signer = ethersProvider.getSigner()
        const message = await signer.signMessage("Sign this message to create your identity!")

        const identity = new ZkIdentity(Strategy.MESSAGE, message)
        const identityCommitment = identity.genIdentityCommitment()
        const identityCommitments = await (await fetch("./identityCommitments.json")).json()

        const merkleProof = generateMerkleProof(20, BigInt(0), identityCommitments, identityCommitment)

        setLogs("Creating your Semaphore proof...")

        const greeting = "Hello world"
        setNewGreeter(identity)

        const witness = Semaphore.genWitness(
            identity.getTrapdoor(),
            identity.getNullifier(),
            merkleProof,
            merkleProof.root,
            greeting
        )

        const { proof, publicSignals } = await Semaphore.genProof(witness, "./semaphore.wasm", "./semaphore_final.zkey")
        const solidityProof = Semaphore.packToSolidityProof(proof)

        const response = await fetch("/api/greet", {
            method: "POST",
            body: JSON.stringify({
                greeting,
                nullifierHash: publicSignals.nullifierHash,
                solidityProof: solidityProof
            })
        })

        if (response.status === 500) {
            const errorMessage = await response.text()

            setLogs(errorMessage)
        } else {
            setLogs("Your anonymous greeting is onchain :)")
        }
    }

    return (
        <div className={styles.container}>
            <Head>
                <title>Greetings</title>
                <meta name="description" content="A simple Next.js/Hardhat privacy application with Semaphore." />
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <main className={styles.main}>
                <h1 className={styles.title}>Greetings</h1>

                <p className={styles.description}>A simple Next.js/Hardhat privacy application with Semaphore.</p>

                <div className={styles.logs}>{logs}</div>

                <div onClick={() => greet()} className={styles.button}>
                    Greet
                </div>
                <div>
                    {newGreeter }
                </div>
                <div>
                    <form onSubmit={onSubmit}>

                        <input {...register("firstName")} placeholder="First Name" />
                        {errors?.firstName && <p>{errors.firstName.message}</p>}
                        <br/>
                        <input {...register("lastName")} placeholder="Last Name" />
                        {errors?.lastName && <p>{errors.lastName.message}</p>}
                        <br/>
                        <input {...register("email")} placeholder="Email" />
                        {errors?.email && <p>{errors.email.message}</p>}
                        <br/>
                        <input {...register("age")} placeholder="Age" />
                        {errors?.age && <p>{errors.age.message}</p>}
                        <br/>
                        <input type="submit" />
                    </form>
                </div>
            </main>
        </div>
    )
}
