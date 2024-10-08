import React, { useState } from "react";
import { motion } from "framer-motion";

const About = () => {
  const [isHover1, setIsHover1] = useState(false);
  const [isHover2, setIsHover2] = useState(false);

  const changeColor1 = () => {
    setIsHover1(!isHover1);
  };
  const changeColor2 = () => {
    setIsHover2(!isHover2);
  };
  return (
    <div id="about">
      <div className="flex justify-center md:pt-[150px] sm:pt-[100px] pt-10 items-center">
        <div className="text-[#8CE605] mb-8 font-poppins font-bold sm:text-2xl md:text-4xl uppercase  tracking-wider">
          About Us
        </div>
      </div>
      <motion.div
        initial={{ y: 200, opacity: 0 }}
        transition={{ duration: 0.5 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true }}
        className="flex sm:flex-col md:flex-row justify-evenly md:pt-[50px] sm:pt-5 items-center md:px-6 lg:px-24"
      >
        <div
          onMouseEnter={changeColor1}
          onMouseLeave={changeColor1}
          className={`md:w-96 border-2 border-[#8CE605] sm:px-4 md:px-10 py-10 sm:mb-4 rounded-lg ${
            isHover1 ? "bg-[#8ce605]/90" : ""
          }`}
        >
          <div className="flex flex-col justify-center items-center">
          <div className="w-[50px] h-[50px] border border-[#ffffff] rounded-full mb-6 overflow-hidden">
            <img
              src="https://cdn.intra.42.fr/users/598e51ff158a507c9698fe7cac8ad26d/lahamoun.JPG"
              alt="what"
              className="w-full h-full object-cover"
            />
          </div>

            <div
              className={`text-[#ffffff] md:mb-8 font-poppins font-bold text-sm ${
                isHover1 ? "text-black" : "text-white"
              }`}
            >
              I'm a computer science student with a passion for blockchain and web3 technologies.
              I joined the web3 space in 2019, and in 2021, I began working as a smart contract auditor, 
              collaborating with 3 NFT projects to ensure their security and integrity. My focus is on improving security
              in decentralized ecosystems, particularly on platforms like Solana.
              <br />
              <br />
              <br />
              <br />
              </div>

            <div
              className={`text-[#8CE605] mt-8 font-Noto font-bold text-xl ${
                isHover1 ? "text-black" : "text-[#8CE605]"
              }`}
            >
              Computer Science Student
            </div>
            <div
              className={`text-[#8CE605] font-Noto font-bold text-xl ${
                isHover1 ? "text-black" : "text-[#8CE605]"
              }`}
            >
              Ucine Hamouni
            </div>
            </div>
            </div>
            <div
            onMouseEnter={changeColor2}
            onMouseLeave={changeColor2}
            className={`md:w-96 border-2 border-[#8CE605] sm:px-4 md:px-10 py-9 sm:mb-4 rounded-lg ${
            isHover2 ? "bg-[#8ce605]/90" : ""
            }`}
            >
            <div className="flex flex-col justify-center items-center">
            <div className="flex flex-col justify-center items-center">
            <div className="w-[50px] h-[50px] border border-[#ffffff] rounded-full mb-6 overflow-hidden">
              <img
                src="https://cdn.intra.42.fr/users/d480a66a1e1d6a67af1374964d9b90a8/raitmous.jpg"
                alt="what"
                className="w-full h-full object-cover"
              />
            </div>
            </div>
            <div
              className={`text-[#ffffff] md:mb-20 font-poppins font-bold text-sm ${
                isHover2 ? "text-black" : "text-white"
              }`}
            >
              I'm a software developer with 2 years of experience in web3 development, 
              I have a deep understanding of blockchain technologies and smart contracts, 
              and I’m passionate about contributing to the web3 ecosystem. My work has 
              involved developing solutions for decentralized finance (DeFi) 
              and NFT projects, with a strong focus on user experience and security.
              </div>
            <div
              className={`text-[#8CE605] mt-8 font-Noto font-bold text-xl ${
                isHover2 ? "text-black" : "text-[#8CE605]"
              }`}
            >
              Software Developer
            </div>
            <div
              className={`text-[#8CE605] font-Noto font-bold text-xl ${
                isHover2 ? "text-black" : "text-[#8CE605]"
              }`}
            >
              Rachid Ait Moussa
            </div>
          </div>
        </div>
      </motion.div>
    </div>
  );
};

export default About;
