import Head from "next/head";
import Image from "next/image";

import Navbar from "../components/Navbar";
import Hero from "../components/Hero";
import Work from "../components/Work";
import Client from "../components/Client";
import About from "../components/About";
import Contact from "../components/Contact";
import Footer from "../components/Footer";
import Terminal from "../components/Terminal";
import ParticlesBackground from "../components/ParticlesBackground";

export default function Home() {
  return (
    <>
      <ParticlesBackground />
      <main className="w-full">
        <section className="md:w-full h-auto">
          <Navbar />
        </section>

        <div className="mx-auto container flex  flex-col justify-center items-center md:px-14">
          <section className="min-h-screen w-full">
            <Hero />
          </section>
          <section className="h-14 w-full"></section>
          <section className="min-h-screen  w-full">
            <Work />
          </section>
          {/* <section className="min-h-screen w-full">
            <Client />
          </section> */}          
          <section id="terminal" className="min-h-screen w-full">
            <Terminal />
          </section>
          <section className="min-h-screen w-full">
            <About />
          </section>
          <section className="min-h-screen mx-auto w-4/5">
            <Contact />
          </section>
        </div>
        <footer className="h-auto w-full">
          <Footer />
        </footer>
      </main>
    </>
  );
}
