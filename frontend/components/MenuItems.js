import React from "react";
import { Link } from "react-scroll";
import CloseIcon from "@mui/icons-material/Close";

const MenuItems = ({ showMenu, active }) => {
  return (
    <ul
      className={
        active
          ? " text-black flex flex-col items-end fixed  right-0 top-[-10px] backdrop-blur-sm bg-[#ffffff]/20 justify-start p-8 md:hidden gap-4 z-10 h-screen w-full transition-all duration-1000"
          : "hidden"
        }
    >
      <div className="translate-x-0 flex flex-col items-end fixed right-[-10px] top-[-2px] justify-start pt-8 pb-24 pl-28 pr-10 md:hidden gap-4 font-semibold  z-30  bg-[#8CE605]  transition-all duration-200 ">
        <CloseIcon onClick={showMenu} className="cursor-pointer" />
        <li>
          <Link
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
            activeClass="active"
            to="work"
            spy={true}
            smooth={true}
            offset={0}
            duration={500}
          >
            What we do
          </Link>
        </li>
        <li>
          <Link
            activeClass="active"
            to="terminal"
            spy={true}
            smooth={true}
            offset={0}
            duration={500}
          >
            Audit
          </Link>
        </li>
        <li>
          <Link
            activeClass="active"
            to="about"
            spy={true}
            smooth={true}
            offset={0}
            duration={500}
          >
            About Us
          </Link>
        </li>
        <li>
          <Link
            activeClass="active"
            to="contact"
            spy={true}
            smooth={true}
            offset={0}
            duration={500}
          >
            Contact
          </Link>
        </li>
      </div>
    </ul>
  );
};

export default MenuItems;
