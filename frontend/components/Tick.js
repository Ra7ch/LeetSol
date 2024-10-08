import React from "react";
import Lottie from "react-lottie";
import animationData from "../public/static/tick_animation.json";

const Tick = () => {
  const defaultOptions = {
    loop: 1,
    autoplay: false,
    animationData: animationData,
    rendererSettings: {
      preserveAspectRatio: "xMidYMid slice",
    },
  };

  return (
    <div>
      <Lottie options={defaultOptions} height={25} width={50} />
    </div>
  );
};

export default Tick;
