import React from "react";
import { useTypewriter, Cursor } from "react-simple-typewriter";
import { motion } from "framer-motion";
import { Link } from "react-scroll";

const Hero = () => {
  const [text] = useTypewriter({
    words: ["Ensuring the future of decentralized finance through secure smart contracts"],
  });
  return (
    <div className="h-full" id="hero">
      <div className="relative flex justify-center items-center h-20">
        <h1 className="absolute sm:top-[150px] text-center md:top-[200px] font-bold uppercase font-oswald sm:text-3xl md:text-5xl lg:text-6xl text-[#8CE605] tracking-widest z-0">
          <span className="motion-safe:animate-pulse">{text}</span>
          <Cursor />
        </h1>
      </div>
      <motion.div
        initial={{ opacity: 0 }}
        transition={{ duration: 0.5 }}
        whileInView={{ opacity: 1 }}
        viewport={{ once: true }}
        className="h-80 flex justify-center items-center"
      >
      <p className="font-Noto text-center tracking-wide font-semibold sm:text-md lg:text-2xl md:text-lg text-stone-200 sm:pt-[100px] lg:pt-28 absolute md:top-[350px] sm:top-[150px] z-0 mx-auto sm:px-8 md:px-12 lg:w-2/3 xl:w-2/3">
          At LEETSOL, we believe that the foundation of a
          secure and scalable Solana ecosystem begins with rigorous 
          smart contract auditing. Our mission is to ensure that every 
          project on Solana is fortified against vulnerabilities and 
          safeguarded for long-term success
      </p>
      </motion.div>
      <div className="flex md:flex-row sm:flex-col justify-around items-center md:mt-32 lg:mt-48 xl:mt-64 2xl:mt-80 md:w-full lg:px-56 md:px-12 md:mx-auto">
      <div className="relative group">
        <Link
          activeClass="active"
          to="work"
          spy={true}
          smooth={true}
          offset={80}
          duration={500}
        >
          <button className="text-[#8CE605] border-2 border-[#8CE605] rounded-[8px] sm:px-10 md:px-12 sm:py-5 sm:mb-5 md:py-5 uppercase font-oswald transition-all hover:scale-105 hover:ease-in-out duration-700">
            Smart Contract Auditing
          </button>
        </Link>
      </div>

      <Link
        activeClass="active"
        to="terminal"
        spy={true}
        smooth={true}
        offset={-100}
        duration={500}
      >
        <button className="text-[#8CE605] border-2 border-[#8CE605] rounded-[8px] sm:px-10 md:px-12 sm:py-5 sm:mb-5 md:py-5 uppercase font-oswald transition-all hover:scale-105 hover:ease-in-out duration-700">
          Start your audit Now
        </button>
      </Link>
    </div>
    </div>
  );
};

export default Hero;
