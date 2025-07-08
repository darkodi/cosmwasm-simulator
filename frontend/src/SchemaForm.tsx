import React, { useEffect, useState } from 'react';
import Form from '@rjsf/core';
import validator from '@rjsf/validator-ajv8';

type Props = {
  schemaPath: string;
  onSubmit: (data: any) => void;
};

export const SchemaForm: React.FC<Props> = ({ schemaPath, onSubmit }) => {
  const [schema, setSchema] = useState<any>(null);

  useEffect(() => {
    const fetchSchema = async () => {
      try {
        const res = await fetch(`/schema/${schemaPath}`);
        const json = await res.json();
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
      onSubmit={({ formData }) => onSubmit(formData)}
    >
      <button type="submit">Simulate</button>
    </Form>
  );
};