import React, { useState } from "react";

import MenuItems from "./MenuItems";
import MenuIcon from "@mui/icons-material/Menu";
import Image from "next/image";
import { Link } from "react-scroll";

const Navbar = () => {
  const [active, setActive] = useState(false);
  const [linkIsActive1, setLinkIsActive1] = useState(false);
  const [linkIsActive2, setLinkIsActive2] = useState(false);
  const [linkIsActive3, setLinkIsActive3] = useState(false);
  const [linkIsActive4, setLinkIsActive4] = useState(false);
  const [linkIsActive5, setLinkIsActive5] = useState(false);

  const onUnderline1 = () => {
    setLinkIsActive1(true);
    setLinkIsActive2(false);
    setLinkIsActive3(false);
    setLinkIsActive4(false);
    setLinkIsActive5(false);
  };

  const onUnderline2 = () => {
    setLinkIsActive1(false);
    setLinkIsActive2(true);
    setLinkIsActive3(false);
    setLinkIsActive4(false);
    setLinkIsActive5(false);
  };
  const onUnderline3 = () => {
    setLinkIsActive1(false);
    setLinkIsActive2(false);
    setLinkIsActive3(true);
    setLinkIsActive4(false);
    setLinkIsActive5(false);
  };
  const onUnderline4 = () => {
    setLinkIsActive1(false);
    setLinkIsActive2(false);
    setLinkIsActive3(false);
    setLinkIsActive4(true);
    setLinkIsActive5(false);
  };
  const onUnderline5 = () => {
    setLinkIsActive1(false);
    setLinkIsActive2(false);
    setLinkIsActive3(false);
    setLinkIsActive4(false);
    setLinkIsActive5(true);
  };

  const showMenu = () => {
    setActive(!active);
  };

  return (
    <div className="sm:fixed h-16 z-50 bg-black w-full text-white font-bold flex justify-around p-4 items-center">
    <div className="cursor-pointer flex justify-center items-center text-[#8CE605] tracking-wider uppercase text-2xl font-bold relative">
      <div className="relative sm:top-3">
        <Link
          activeClass="active"
          to="hero"
          spy={true}
          smooth={true}
          offset={50}
          duration={500}
        >
        <Image
          alt="Next.js logo"
          src="/static/leetsologo.png"
          width={300}
          height={65}
          />
        </Link>
      </div>
    </div>



      <nav className="cursor-pointer">
        <div className="sm:absolute right-8 md:hidden top-5 scale-100">
          <MenuIcon onClick={showMenu} className=" cursor-pointer" />
        </div>
        <ul className="hidden md:flex gap-8 p-6 uppercase">
          <li>
            <Link
              onClick={onUnderline1}
              className={`text-slate-300 hover:text-white ${
                linkIsActive1 ? "underline" : ""
              }`}
              activeClass="active"
              to="hero"
              spy={true}
              smooth={true}
              offset={50}
              duration={500}
            >
              Home
            </Link>
          </li>
          <li>
            <Link
              onClick={onUnderline2}
              className={`text-slate-300 hover:text-white ${
                linkIsActive2 ? "underline" : ""
              }`}
              activeClass="active"
              to="work"
              spy={true}
              smooth={true}
              offset={50}
              duration={500}
            >
              What we do
            </Link>
          </li>
          <li className="text-slate-300 hover:text-white">
            <Link
              onClick={onUnderline3}
              className={`text-slate-300 hover:text-white ${
                linkIsActive3 ? "underline" : ""
              }`}
              activeClass="active"
              to="terminal"
              spy={true}
              smooth={true}
              offset={-50}
              duration={500}
            >
              Audit
            </Link>
          </li>
          <li className="text-slate-300 hover:text-white">
            <Link
              onClick={onUnderline4}
              className={`text-slate-300 hover:text-white ${
                linkIsActive4 ? "underline" : ""
              }`}
              activeClass="active"
              to="about"
              spy={true}
              smooth={true}
              offset={50}
              duration={500}
            >
              About Us
            </Link>
          </li>
          <li className="text-slate-300 hover:text-white z-50">
            <Link
              onClick={onUnderline5}
              className={`text-slate-300 hover:text-white ${
                linkIsActive5 ? "underline" : ""
              }`}
              activeClass="active"
              to="contact"
              spy={true}
              smooth={true}
              offset={50}
              duration={500}
            >
              Contact Us
            </Link>
          </li>
        </ul>
        <MenuItems showMenu={showMenu} active={active} />
      </nav>
    </div>
  );
};

export default Navbar;
