import {
  Theme,
  Card,
  Flex,
  Heading,
  Text,
  Box,
  Avatar,
} from "@radix-ui/themes";
import { render } from "react-dom";
import React from "react";
function TopCustomers() {
  return (
    <Theme>
      <Card size="3" className="w-96">
        <Flex direction="column" gap="3">
          <Heading as="h4" size="3" className="mb-1">
            Top Customers
          </Heading>
          <Text as="p" size="2" className="mb-4" color="gray">
            Your top customers based on total revenue
          </Text>
          <Box className="space-y-3">
            <Flex align="center" gap="2">
              <Avatar
                className="w-8 h-8 mr-2"
                fallback="A"
                radius="full"
                color="blue"
              />
              <Flex direction="column" gap="1">
                <Heading as="h5" size="2" className="leading-none">
                  Alice
                </Heading>
                <Text as="p" size="1" color="gray">
                  alice@example.com
                </Text>
              </Flex>
              <Flex
                direction="column"
                align="center"
                gap="1"
                className="ml-auto"
              >
                <Text as="p" size="1" color="gray">
                  120 orders
                </Text>
              </Flex>
            </Flex>
            <Flex align="center" gap="2">
              <Avatar
                className="w-8 h-8 mr-2"
                fallback="B"
                radius="full"
                color="red"
              />
              <Flex direction="column" gap="1">
                <Heading as="h5" size="2" className="leading-none">
                  Bob
                </Heading>
                <Text as="p" size="1" color="gray">
                  bob@example.com
                </Text>
              </Flex>
              <Flex
                direction="column"
                align="center"
                gap="1"
                className="ml-auto"
              >
                <Text as="p" size="1" color="gray">
                  97 orders
                </Text>
              </Flex>
            </Flex>
            <Flex align="center" gap="2">
              <Avatar
                className="w-8 h-8 mr-2"
                fallback="C"
                radius="full"
                color="green"
              />
              <Flex direction="column" gap="1">
                <Heading as="h5" size="2" className="leading-none">
                  Carol
                </Heading>
                <Text as="p" size="1" color="gray">
                  carol@example.com
                </Text>
              </Flex>
              <Flex
                direction="column"
                align="center"
                gap="1"
                className="ml-auto"
              >
                <Text as="p" size="1" color="gray">
                  89 orders
                </Text>
              </Flex>
            </Flex>
          </Box>
        </Flex>
      </Card>
    </Theme>
  );
}
render(<TopCustomers />, document.getElementById("root"));
