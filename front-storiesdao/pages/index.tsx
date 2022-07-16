import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'

import sdk from '../sdk';
import createHomeViewmodel from '../viewmodels/home.js';

const Home: NextPage = () => {
  const vm = createHomeViewmodel(sdk);

  return (
    <div className={styles.container}>
      <Head>
        <title>StoriesDAO</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
      </main>
    </div>
  )
}

export default Home
