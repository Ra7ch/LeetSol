import React, { useRef, useState } from "react";
import emailjs from "@emailjs/browser";

const Contact = () => {

  const [isButtonText, setIsButtonText] = useState("SEND MESSAGE");
  const form = useRef();

  const checkValidationEmail = (e) => {
    e.preventDefault();
    if (!e.target.value.includes("@")) {
      alert("Enter Valid Email");
    }
  };

  const SendEmail = (e) => {
    e.preventDefault();

    emailjs
      .sendForm(
        `${process.env.NEXT_PUBLIC_SERVICE_ID}`,
        `${process.env.NEXT_PUBLIC_TEMPLATE_ID}`,
        form.current,
        `${process.env.NEXT_PUBLIC_PUBLIC_KEY}`
      )
      .then(
        (result) => console.log(result.text),
        (error) => console.log(error.text)
      );
    setIsButtonText(`MESSAGE SENT `);

    e.target.reset();
  };

  return (
    <div id="contact">
      <div className="flex justify-center md:pt-[150px] sm:pt-[120px]  items-center">
        <div className="text-[#8CE605] mb-8 font-poppins font-bold sm:text-2xl md:text-4xl uppercase tracking-wider">
          Contact Us
        </div>
      </div>
      <div className="flex justify-center items-center text-[#ffffff] lg:py-6 rounded-lg ">
        <form ref={form} onSubmit={SendEmail}>
          <div className="flex md:flex-row sm:flex-col items-center md:justify-between md:w-full ">
            <div className="flex items-center w-full ">
              <label className="sm:mr-[2px] md:mx-2" htmlFor="name">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  strokeWidth={1.5}
                  stroke="currentColor"
                  className="w-6 h-6"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125"
                  />
                </svg>
              </label>

              <div className=" bg-neutral-900 sm:mb-3 md:mb-0 rounded-lg w-full">
                <input
                  className="bg-transparent ml-2 outline-0 md:w-96 sm:w-60
                    md:pl-4 py-2  sm:mx-6 md:mx-3 sm:my-2  rounded-lg"
                  id="name"
                  type="text"
                  name="user_name"
                  placeholder="Name"
                  required
                />
              </div>
            </div>
            <div className="flex items-center w-full">
              <label className="sm:mr-[2px] md:mx-2" htmlFor="email">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  strokeWidth={1.5}
                  stroke="currentColor"
                  className="w-6 h-6"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M21.75 9v.906a2.25 2.25 0 01-1.183 1.981l-6.478 3.488M2.25 9v.906a2.25 2.25 0 001.183 1.981l6.478 3.488m8.839 2.51l-4.66-2.51m0 0l-1.023-.55a2.25 2.25 0 00-2.134 0l-1.022.55m0 0l-4.661 2.51m16.5 1.615a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V8.844a2.25 2.25 0 011.183-1.98l7.5-4.04a2.25 2.25 0 012.134 0l7.5 4.04a2.25 2.25 0 011.183 1.98V19.5z"
                  />
                </svg>
              </label>

              <div className=" bg-neutral-900 rounded-lg w-full">
                <input
                  className="bg-transparent ml-2 outline-0 md:w-96 sm:w-60 md:pl-4 py-2 sm:mx-6 md:mx-3 sm:my-2 rounded-lg"
                  id="email"
                  type="email"
                  name="user_email"
                  placeholder="Email Address"
                  required
                  onBlur={checkValidationEmail}
                />
              </div>
            </div>
          </div>
          <div className="flex md:flex-row  sm:flex-col items-center md:justify-between mt-3  sm:w-72   md:w-full">
            <div className="flex items-center w-full">
              <label className="sm:mr-[2px] md:mx-2" htmlFor="phone">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  strokeWidth={1.5}
                  stroke="currentColor"
                  className="w-6 h-6"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M10.5 1.5H8.25A2.25 2.25 0 006 3.75v16.5a2.25 2.25 0 002.25 2.25h7.5A2.25 2.25 0 0018 20.25V3.75a2.25 2.25 0 00-2.25-2.25H13.5m-3 0V3h3V1.5m-3 0h3m-3 18.75h3"
                  />
                </svg>
              </label>

              <div className=" bg-neutral-900 sm:mb-3 md:mb-0 rounded-lg w-full">
                <input
                  className="bg-transparent ml-2 outline-0 md:w-96 sm:w-60  md:pl-4 py-2  sm:mx-6 md:mx-3 sm:my-2  rounded-lg"
                  id="phone"
                  type="number"
                  name="user_phone"
                  placeholder="Phone Number"
                  required
                />
              </div>
            </div>
            <div className="flex items-center w-full ">
              <label className="sm:mr-[2px] md:mx-2" htmlFor="subject">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  strokeWidth={1.5}
                  stroke="currentColor"
                  className="w-6 h-6"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M9 3.75H6.912a2.25 2.25 0 00-2.15 1.588L2.35 13.177a2.25 2.25 0 00-.1.661V18a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 00-2.15-1.588H15M2.25 13.5h3.86a2.25 2.25 0 012.012 1.244l.256.512a2.25 2.25 0 002.013 1.244h3.218a2.25 2.25 0 002.013-1.244l.256-.512a2.25 2.25 0 012.013-1.244h3.859M12 3v8.25m0 0l-3-3m3 3l3-3"
                  />
                </svg>
              </label>
              <div className=" bg-neutral-900 sm:mb-3 md:mb-0 rounded-lg w-full">
                <input
                  className="bg-transparent ml-2 outline-0 md:w-96 sm:w-60 md:pl-4 py-2  sm:mx-6 md:mx-3 sm:my-2  rounded-lg"
                  id="subject"
                  type="text"
                  name="user_subject"
                  placeholder="Subject"
                  required
                />
              </div>
            </div>
          </div>
          <div className="flex flex-col items-center justify-around mt-3  sm:w-full md:w-full">
            <div className="flex items-start w-full">
              <label className="sm:mr-[2px] md:mx-2 pt-3" htmlFor="message">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  strokeWidth={1.5}
                  stroke="currentColor"
                  className="w-6 h-6"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M12 7.5h1.5m-1.5 3h1.5m-7.5 3h7.5m-7.5 3h7.5m3-9h3.375c.621 0 1.125.504 1.125 1.125V18a2.25 2.25 0 01-2.25 2.25M16.5 7.5V18a2.25 2.25 0 002.25 2.25M16.5 7.5V4.875c0-.621-.504-1.125-1.125-1.125H4.125C3.504 3.75 3 4.254 3 4.875V18a2.25 2.25 0 002.25 2.25h13.5M6 7.5h3v3H6v-3z"
                  />
                </svg>
              </label>
              <div className="flex justify-center items-center w-full">
                <div className="bg-neutral-900 rounded-lg w-full ">
                  <textarea
                    id="message"
                    name="message"
                    rows="4"
                    className="text-white outline-0 block p-4 text-sm  bg-neutral-900 md:w-full sm:w-[284px]  sm:h-[150px] md:h-[220px] rounded-lg"
                    placeholder="Your Message..."
                    required
                  ></textarea>
                </div>
              </div>
            </div>
            <div className="mt-6">
              <div className="relative group">
                <div className="absolute -inset-0.5 opacity-75 blur rounded-[8px] h-16  sm:px-10  md:px-12  sm:py-5 sm:mb-5 md:py-5 bg-[#f3de2c]  group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-tilt "></div>
                <button
                  type="submit"
                  value="send"
                  className="h-16 relative sm:w-60 md:w-72 mb-5 text-black border border-white bg-[#f3de2c] rounded-lg sm:px-5  md:px-12  sm:py-5 sm:mb-5 md:py-5   uppercase font-oswald transition-all  hover:scale-110 hover:ease-in-out duration-700"
                >
                  {isButtonText}
                </button>
              </div>
            </div>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Contact;