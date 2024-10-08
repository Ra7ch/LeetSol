import React, { useState } from "react";
import Image from "next/image";
import ExpandCircleDownIcon from "@mui/icons-material/ExpandCircleDown";
import { motion } from "framer-motion";

const Work = () => {
  const [isHover1, setIsHover1] = useState(false);
  const [isHover2, setIsHover2] = useState(false);
  const [isShow, setIsShow] = useState(true);
  const [isShow1, setIsShow1] = useState(true);

  const changeColor1 = () => {
    setIsHover1(true);
  };
  const changeColorToWhite1 = () => {
    setIsHover1(false);
  };
  const changeColor2 = () => {
    setIsHover2(true);
  };
  const changeColorToWhite2 = () => {
    setIsHover2(false);
  };

  const showDiv = () => {
    setIsShow(!isShow);
  };
  const showDiv1 = () => {
    setIsShow1(!isShow1);
  };

  return (
    <div
      className="flex flex-col justify-center md:pt-[150px] sm:pt-[120px] items-center"
      id="work"
    >
      <div className="text-[#8CE605] mb-8 font-poppins font-bold sm:text-2xl md:text-4xl uppercase tracking-wider">
        What We Do
      </div>
      <div className="flex flex-col justify-center items-center">
        <motion.div
          onClick={showDiv}
          initial={{ y: 200, opacity: 0 }}
          transition={{ duration: 0.25 }}
          whileInView={{ y: 0, opacity: 1 }}
          viewport={{ once: true }}
          onMouseEnter={changeColor1}
          onMouseLeave={changeColorToWhite1}
          className="border-2  bg-black/40 border-[#8CE605] sm:px-5 md:px-10 md:py-10 sm:py-5 rounded-lg mb-6 hover:bg-[#8CE605]/50 transition duration-700"
        >
          <div className="flex md:flex-row sm:flex-col sm:justify-center md:justify-self-auto sm:items-center   md:items-start lg:w-[900px] md:w-[600px] sm:w-[300px]">
            <div className="md:w-[700px] sm:w-[100px] sm:pb-2  md:pr-4 ">
              <Image
                alt="Next.js logo"
                src="/static/asset1.png"
                width={120}
                height={120}
              />
            </div>
            <div className="flex justify-center items-center ">
              <div
                className={`sm:text-sm md:text-md font-Noto font-semibold ${
                  isHover1 ? "text-white" : "text-white"
                }`}
              >
                <h1 className="text-xl">Smart Contract Auditing</h1>
                {isShow ? (
                  <>
                    <span>
                    we believe that smart contracts are the building blocks of the future decentralized world.
                    However, with great power comes great responsibility. That's why we focus on delivering a 
                    comprehensive and accurate audit to secure every aspect of your contract. Our specialized 
                    audit process is designed to help secure your Solana project, earning trust and confidence 
                    from investors, developers, and the entire community.
                    </span>
                  </>
                ) : (
                  <div>
                    we believe that smart contracts are the building blocks of the future decentralized world.
                    However, with great power comes great responsibility. That's why we focus on delivering a 
                    comprehensive and accurate audit to secure every aspect of your contract. Our specialized 
                    audit process is designed to help secure your Solana project, earning trust and confidence 
                    from investors, developers, and the entire community.
                    <div>
                      <br />
                      <h1 className="text-xl">Automated Tools</h1>
                      <p className="text-md">
                      Our auditing journey begins with our specially developed LEETSOL Audit Engine.
                      This automated tool rigorously scans through the uploaded smart contracts to 
                      pinpoint potential vulnerabilities commonly found in Solana-based projects.
                      The engine analyses your contracts line by line so that it scans through each
                      and every instruction, statement, variable and function.
                      </p>
                      <br />
                      <h1 className="text-xl">Security Stress Testing (Advanced - Contact Us for Details)</h1>
                      <p className="text-md">
                      Take your security to the next level. In this advanced iteration, our team conducts rigorous testing to uncover even the most hidden vulnerabilities.
                      This premium service is ideal for projects looking for the highest level of assurance.
                      Extreme Condition Testing: We simulate real-world attacks to test your contract under extreme conditions, ensuring resilience against complex threats.
                      <br />
                      Fuzz Testing & Mainnet Simulation: By using fuzz testing and mimicking real mainnet interactions, we uncover potential edge cases and unintended behaviors.
                      <br />
                      Intention vsAction Analysis: Ensuring the intended functionality matches actual behavior, leaving no room for unexpected outcomes.
                      For more details or to access this advanced level of auditing, contact us to discuss your needs and receive a custom quote tailored to your project’s scope.
                      </p>
                      <br />
                
                    </div>
                  </div>
                )}
              </div>
              <div className="z-40 ml-3 text-white w-10 h-10 scale-150">
                {isShow ? (
                  <div>
                    <ExpandCircleDownIcon />
                  </div>
                ) : (
                  <div onClick={changeColor1} className="rotate-180">
                    <ExpandCircleDownIcon />
                  </div>
                )}
              </div>
            </div>
          </div>
        </motion.div>
        <motion.div
          onClick={showDiv1}
          initial={{ y: 200, opacity: 0 }}
          transition={{ duration: 0.25 }}
          whileInView={{ y: 0, opacity: 1 }}
          viewport={{ once: true }}
          onMouseEnter={changeColor2}
          onMouseLeave={changeColorToWhite2}
          className="border-2  bg-black/40 border-[#8CE605] sm:px-5 md:px-10 md:py-10 sm:py-5 rounded-lg mb-6 hover:bg-[#8CE605]/50 transition duration-700"
        >
          <div className="flex md:flex-row sm:flex-col sm:justify-center md:justify-self-auto sm:items-center   md:items-start lg:w-[900px] md:w-[600px] sm:w-[300px]">
            <div className="md:w-[500px] sm:w-[100px] sm:pb-2  md:pr-4 ">
              <Image
                alt="Next.js logo"
                src="/static/asset2.png"
                width={120}
                height={120}
              />
            </div>
            <div className="flex justify-center items-center ">
              <div
                className={`sm:text-sm md:text-md font-Noto font-semibold ${
                  isHover2 ? "text-white" : "text-white"
                }`}
              >
                <h1 className="text-xl">AI-Powered Auditing - Trained AI Auditor (Soon)
                </h1>
                {isShow1 ? (
                  <>
                    <span>
                    we're pushing the boundaries of blockchain security. In the future,
                     our auditing engine will integrate a powerful AI, revolutionizing the way smart contracts are audited.
                    </span>
                  </>
                ) : (
                  <div>
                    we're pushing the boundaries of blockchain security. In the future, our auditing engine will integrate a powerful AI, revolutionizing the way smart contracts are audited.
                    <br />
                    <br />
                    <h1 className="text-xl"> Here’s what our AI-powered audit will offer</h1>
                    
                    <div>
                      <br />
                      <li className="text-md">Automated Deep Analysis: The AI will conduct in-depth, line-by-line analysis, significantly reducing human error and delivering more precise results in record time.</li>
                      <li className="text-md">
                      Learning from Attacks: Using machine learning, the AI will continually learn from new attack vectors and vulnerabilities, staying ahead of emerging threats.
                      </li>
                      <li className="text-md">
                      Risk Scoring & Prediction: The AI will assign risk scores to vulnerabilities, predict potential exploits, and provide remediation recommendations based on millions of data points.
                      </li>
                      <li className="text-md">
                      Intelligent Bug Detection & Reporting: The AI will not only identify standard issues but will also learn from historical audits, enabling it to detect less obvious vulnerabilities and generate comprehensive reports.
                      </li>
                      <li className="text-md">
                      Cost & Efficiency: This future AI integration will reduce the time and cost associated with manual audits, providing you with faster and more affordable security without compromising on quality.
                      </li>
                     
                    </div>
                  </div>
                )}
              </div>
              <div className="z-40 ml-3 text-white w-10 h-10 scale-150">
                {isShow1 ? (
                  <div>
                    <ExpandCircleDownIcon />
                  </div>
                ) : (
                  <div onClick={changeColor2} className="rotate-180">
                    <ExpandCircleDownIcon />
                  </div>
                )}
              </div>
            </div>
          </div>
        </motion.div>
      </div>
    </div>
  );
};

export default Work;
