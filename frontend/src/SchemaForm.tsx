import React, { useEffect, useState } from 'react';
import Form from '@rjsf/core';
import validator from '@rjsf/validator-ajv8';
import CustomFieldTemplate from './CustomFieldTemplate';

type Props = {
  schemaPath: string;
  onSubmit: (data: any) => void;
};

export const SchemaForm: React.FC<Props> = ({ schemaPath, onSubmit }) => {
  const [schema, setSchema] = useState<any>(null);

  // Define allowed action keys per schema
  const allowedActions: Record<string, string[]> = {
    'cw20_base/execute_msg.json': ['transfer', 'burn'],
    // Add more schema paths and actions here if needed
  };

  useEffect(() => {
    const fetchSchema = async () => {
      try {
        const res = await fetch(`/schema/${schemaPath}`);
        const json = await res.json();

        // Filter oneOf options if an allowlist is defined
        if (json.oneOf && Array.isArray(json.oneOf)) {
          const allowed = allowedActions[schemaPath];
          if (allowed) {
            json.oneOf = json.oneOf.filter((entry: any) =>
              allowed.some((key) => key in (entry.properties || {}))
            );
          }
        }

        setSchema(json);
      } catch (err) {
        console.error(`❌ Failed to load schema ${schemaPath}`, err);
      }
    };

    fetchSchema();
  }, [schemaPath]);

  if (!schema) {
    return <p>Loading schema...</p>;
  }

  return (
    <Form
      schema={schema}
      validator={validator}
      templates={{ FieldTemplate: CustomFieldTemplate }}
      onSubmit={({ formData }) => onSubmit(formData)}
    >
      <button type="submit">Simulate</button>
    </Form>
  );
};
