import React from 'react';

const CustomFieldTemplate: React.FC<any> = (props) => {
  const {
    id,
    classNames,
    label,
    help,
    required,
    errors,
    children,
    // description intentionally omitted
  } = props;

  return (
    <div className={classNames}>
      {label && (
        <label htmlFor={id}>
          {label}
          {required ? ' *' : ''}
        </label>
      )}
      {children}
      {errors}
      {help}
    </div>
  );
};

export default CustomFieldTemplate;
