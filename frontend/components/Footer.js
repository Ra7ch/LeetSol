import React, { useState } from "react";
import FacebookIcon from "@mui/icons-material/Facebook";
import TwitterIcon from "@mui/icons-material/Twitter";
import LinkedInIcon from "@mui/icons-material/LinkedIn";

const Footer = () => {
  return (
    <div className="">
      <div className="flex sm:flex-col md:flex-row text-gray-600 justify-around items-center pt-32">
        <div className="flex sm:pr-2">
          <div className={`sm:px-1 md:px-3 cursor-pointer  hover:text-white`}>
            <FacebookIcon />
          </div>
          <div className="sm:px-1 md:px-3 cursor-pointer  hover:text-white">
            <TwitterIcon />
          </div>
          <div className="sm:px-1 md:px-3 cursor-pointer hover:text-white">
            <LinkedInIcon />
          </div>
        </div>
        <div className="sm: pt-3 flex flex-col text-center font-semibold ">
          <div>Terms and Conditions</div>
          <div>Privacy Policy</div>
        </div>
        <div className="text-center flex flex-col font-semibold">
          <div>Email:</div>
          <a
            href="mailto:leetsolana@gmail.com"
            className=" hover:text-white"
          >
            leetsolana@gmail.com
          </a>
        </div>
      </div>
      <div className="h-1 border-b border-gray-600 pb-8"></div>
      <div className=" border-b  border-gray-600 pb-1"></div>
      <div className="h-10 flex justify-center items-center">
        <div className="text-gray-600 ">
          Copyright © 2024 LEETSOL. All rights reserved.
        </div>
      </div>
    </div>
  );
};

export default Footer;
