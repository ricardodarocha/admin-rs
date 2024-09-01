import {
  Box,
  Container,
  TextInput,
  Button,
  MantineProvider,
  Text,
} from "@mantine/core";
import { render } from "react-dom";
import React, { useState, useEffect, useRef } from "react";
const LoginPage = () => {
  return (
    <MantineProvider>
      <Box
        style={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          minHeight: "100vh",
        }}
      >
        <Container size="sm">
          <Text style={{ fontWeight: 'bold', marginBottom: '10px'}}>Log into Acme Corp</Text>
          <form>
            <TextInput
              label="Username"
              placeholder="Enter your username"
              style={{
                marginBottom: "10px",
              }}
            />
            <TextInput
              label="Password"
              type="password"
              placeholder="Enter your password"
              style={{
                marginBottom: "10px",
              }}
            />
            <Button type="submit" fullWidth>
              Login
            </Button>
          </form>
        </Container>
      </Box>
    </MantineProvider>
  );
};
export default LoginPage;
render(<LoginPage />, document.getElementById("root"));
