import { useState } from "react";
import { motion } from "framer-motion";

const Client = () => {
  const [isHover1, setIsHover1] = useState(false);
  const [isHover2, setIsHover2] = useState(false);
  const [isHover3, setIsHover3] = useState(false);

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
  const changeColor3 = () => {
    setIsHover3(true);
  };
  const changeColorToWhite3 = () => {
    setIsHover3(false);
  };

  useEffect(() => {}, []);
  let boxVariants = {};

  if (typeof window !== "undefined" && window.innerWidth < 768) {
    boxVariants = {
      transition: {
        duration: 0.75,
      },
      opacity: {
        initial: 0,
        whileInView: 1,
      },
    };
  }

  return (
    <div id="client">
      <div className="flex justify-center md:pt-[150px] sm:pt-[120px] pt-10 items-center">
        <div className="text-[#8CE605] mb-8 font-poppins font-bold sm:text-2xl md:text-4xl uppercase tracking-wider">
          Testimonials
        </div>
      </div>

      {/* rest */}
      <div className="flex sm:w-auto md:w-full sm:overflow-x-auto md:overflow-hidden  md:justify-around items-center">
        <div
          onMouseEnter={changeColor1}
          onMouseLeave={changeColorToWhite1}
          className=" sm:mr-3 md:w-[300px] sm:w-[600px] lg:w-[350px] md:h-[420px] lg:h-[500px] border border-black rounded-md bg-neutral-900 hover:bg-[#8CE605]/80 transition-all duration-1000 flex flex-col justify-center items-center md:p-12  sm:py-8 "
        >
          <div className="w-[50px] h-[50px] border border-[#ffffff] rounded-full mb-2">
            <image
              src="https://avatars.dicebear.com/api/adventurer/your-custom-seed.svg"
              alt="name"
            />
          </div>
          <div className="sm:px-20">
            <div
              className={`text-[#8CE605]  font-Noto font-bold text-sm text-center ${
                isHover1 ? "text-white" : "text-[#8CE605]"
              }`}
            >
              AMANDA LEE
            </div>
            <div
              className={`font-Noto font-bold text-xl text-center ${
                isHover1 ? "text-white" : "text-[#8CE605]"
              }`}
            >
              CEO <span className="italic">@PeckShield</span>
            </div>
          </div>
          <div className="text-[#ffffff] md:mb-8 font-poppins font-bold text-sm md:text-center mt-8 px-6 lg:w-[280px]">
            It is a long established fact that a reader will be distracted by
            the readable content of a page when looking at its layout. The point
            of using Lorem Ipsum is that it has a more-or-less normal
          </div>
        </div>
        <div
          onMouseEnter={changeColor2}
          onMouseLeave={changeColorToWhite2}
          className=" sm:mr-3 md:w-[300px] sm:w-[600px] lg:w-[350px] md:h-[420px] lg:h-[500px] border border-black rounded-md  bg-neutral-900 hover:bg-[#8CE605]/80 transition-all duration-1000 flex flex-col justify-center items-center md:p-12  sm:py-8 "
        >
          <div className="w-[50px] h-[50px] border border-[#ffffff] rounded-full mb-2">
            <image
              src="https://avatars.dicebear.com/api/adventurer/your-custom-seed.svg"
              alt="name"
            />
          </div>
          <div className="sm:px-20">
            <div
              className={`text-[#8CE605]  font-Noto font-bold text-sm text-center ${
                isHover2 ? "text-white" : "text-[#8CE605]"
              }`}
            >
              AMANDA LEE
            </div>
            <div
              className={`font-Noto font-bold text-xl text-center ${
                isHover2 ? "text-white" : "text-[#8CE605]"
              }`}
            >
              CEO <span className="italic">@PeckShield</span>
            </div>
          </div>
          <div className="text-[#ffffff] md:mb-8 font-poppins font-bold text-sm md:text-center mt-8 px-6 lg:w-[280px]">
            It is a long established fact that a reader will be distracted by
            the readable content of a page when looking at its layout. The point
            of using Lorem Ipsum is that it has a more-or-less normal
          </div>
        </div>
        <div
          onMouseEnter={changeColor3}
          onMouseLeave={changeColorToWhite3}
          className=" sm:mr-5 md:w-[300px] sm:w-[600px] lg:w-[350px] md:h-[420px] lg:h-[500px] border border-black rounded-md  bg-neutral-900 hover:bg-[#8CE605]/80 transition-all duration-1000 flex flex-col justify-center items-center md:p-12  sm:py-8 "
        >
          <div className="w-[50px] h-[50px] border border-[#ffffff] rounded-full mb-2">
            <image
              src="https://avatars.dicebear.com/api/adventurer/your-custom-seed.svg"
              alt="name"
            />
          </div>
          <div className="sm:px-20">
            <div
              className={`text-[#8CE605]  font-Noto font-bold text-sm text-center ${
                isHover3 ? "text-white" : "text-[#8CE605]"
              }`}
            >
              AMANDA LEE
            </div>
            <div
              className={`font-Noto font-bold text-xl text-center ${
                isHover3 ? "text-white" : "text-[#8CE605]"
              }`}
            >
              CEO <span className="italic">@PeckShield</span>
            </div>
          </div>
          <div className="text-[#ffffff] md:mb-8 font-poppins font-bold text-sm md:text-center mt-8 px-6 lg:w-[280px]">
            It is a long established fact that a reader will be distracted by
            the readable content of a page when looking at its layout. The point
            of using Lorem Ipsum is that it has a more-or-less normal
          </div>
        </div>
      </div>
    </div>
  );
};

export default Client;
