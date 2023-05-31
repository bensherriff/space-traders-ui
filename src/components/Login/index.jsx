import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import PropTypes from 'prop-types';
import { SessionStorage } from "../../js";
import { useNavigate } from "react-router-dom";

export default function Login( { setAgent } ) {
  const [token, setToken] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  const navigate = useNavigate();

  useEffect(() => {
    var token = SessionStorage.getSessionToken();
    if (token) {
      setToken(token);
    }
  }, [])

  async function get_my_agent() {
    let response = await invoke("get_my_agent", { token: token });
    if (response.data) {
      setErrorMessage("");
      setToken(token);
      SessionStorage.setSessionToken(token);
      SessionStorage.setSessionAgent(response.data);
      setAgent(response.data);
      navigate("/fleet");
    } else if (response.error) {
      setErrorMessage(response.error.message);
    }
  }

  return (
    <div className="m-0 pt-10 flex flex-col justify-center text-center">
      <h1 className="text-5xl mb-4 select-none">Space Traders</h1>

      <div className="flex justify-center">
        <LoginLogo link={"https://spacetraders.io"} src={"/space-traders.svg"} alt={"Space Traders logo"}/>
        <LoginLogo link={"https://github.com/bensherriff/space-traders-ui"} src={"/github-mark-white.svg"} alt={"Github logo"}/>
      </div>

      <form
        className="mt-10 row flex flex-col justify-center items-center"
        onSubmit={(e) => {
          e.preventDefault();
          get_my_agent();
        }}
      >
        <textarea
          className="w-10/12 h-60"
          id="token-input"
          onChange={(e) => setToken(e.currentTarget.value)}
          placeholder="Enter your token..."
        />
        <button className="w-11/12 mt-10 bg-gray-600" type="submit">Connect</button>
      </form>

      <p>{errorMessage}</p>
    </div>
  );
}

function LoginLogo({ link, src, alt }) {
  return (
    <a href={link} target="_blank">
      <img src={src} className="logo scale-150 mx-6" alt={alt}/>
    </a>
  )
}

Login.propTypes = {
  setAgent: PropTypes.func.isRequired
}
